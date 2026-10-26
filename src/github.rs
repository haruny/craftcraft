//! Each app's latest release, from the GitHub API.
//!
//! One request per app, revalidated with ETags (a `304 Not Modified` doesn't count against
//! GitHub's limit) and cached on disk, so a check is cheap and the last answer is shown at once.
//! Without a token GitHub allows 60 requests an hour; when that runs out the launcher shows the
//! cached answers and says when it will try again.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Mutex, PoisonError};

use serde::{Deserialize, Serialize};

use crate::catalog::glob_match;
use crate::http::{Fetched, Http};
use crate::now;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Release {
    pub tag: String,
    pub published_at: String,
    pub html_url: String,
    pub assets: Vec<Asset>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    pub size: u64,
    pub url: String,
    /// Lowercase hex, from GitHub's `digest` field.
    pub sha256: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Lookup {
    Found(Release),
    NoRelease,
    Failed(String),
}

#[derive(Deserialize)]
struct GhRelease {
    tag_name: String,
    #[serde(default)]
    published_at: Option<String>,
    #[serde(default)]
    html_url: String,
    #[serde(default)]
    assets: Vec<GhAsset>,
}

#[derive(Deserialize)]
struct GhAsset {
    name: String,
    #[serde(default)]
    size: u64,
    browser_download_url: String,
    #[serde(default)]
    digest: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
struct Entry {
    etag: Option<String>,
    release: Option<Release>,
}

pub struct Releases {
    http: Http,
    path: PathBuf,
    cache: Mutex<BTreeMap<String, Entry>>,
    limited_until: Mutex<Option<i64>>,
}

impl Releases {
    pub fn new(http: Http, path: PathBuf) -> Self {
        let cache = std::fs::read(&path).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default();
        Self { http, path, cache: Mutex::new(cache), limited_until: Mutex::new(None) }
    }

    pub fn cached(&self, repo: &str) -> Option<Release> {
        self.cache.lock().unwrap_or_else(PoisonError::into_inner).get(repo).and_then(|e| e.release.clone())
    }

    /// When GitHub's request limit lifts, while it is in force.
    pub fn rate_limited_until(&self) -> Option<i64> {
        self.limited_until.lock().unwrap_or_else(PoisonError::into_inner).filter(|t| *t > now())
    }

    pub fn latest(&self, repo: &str) -> Lookup {
        if self.rate_limited_until().is_some() {
            return self.cached_or(repo, "GitHub's request limit was reached");
        }
        let saved = self.cache.lock().unwrap_or_else(PoisonError::into_inner).get(repo).cloned();
        let url = format!("https://api.github.com/repos/{repo}/releases/latest");
        match self.http.get_text(&url, saved.as_ref().and_then(|e| e.etag.as_deref())) {
            Ok(Fetched::Body { text, etag }) => match parse_release(&text, repo) {
                Ok(release) => {
                    self.store(repo, Entry { etag, release: Some(release.clone()) });
                    Lookup::Found(release)
                }
                Err(e) => Lookup::Failed(e),
            },
            Ok(Fetched::NotModified) => match saved.and_then(|e| e.release) {
                Some(release) => Lookup::Found(release),
                None => Lookup::NoRelease,
            },
            Ok(Fetched::NotFound) => {
                self.store(repo, Entry::default());
                Lookup::NoRelease
            }
            Ok(Fetched::RateLimited { reset }) => {
                *self.limited_until.lock().unwrap_or_else(PoisonError::into_inner) = Some(reset.unwrap_or_else(|| now() + 15 * 60));
                self.cached_or(repo, "GitHub's request limit was reached")
            }
            Err(e) => self.cached_or(repo, &e),
        }
    }

    fn cached_or(&self, repo: &str, why: &str) -> Lookup {
        self.cached(repo).map_or_else(|| Lookup::Failed(why.to_string()), Lookup::Found)
    }

    fn store(&self, repo: &str, entry: Entry) {
        self.cache.lock().unwrap_or_else(PoisonError::into_inner).insert(repo.to_string(), entry);
    }

    pub fn save(&self) {
        let bytes = serde_json::to_vec(&*self.cache.lock().unwrap_or_else(PoisonError::into_inner));
        if let Ok(bytes) = bytes
            && let Err(e) = crate::paths::write_atomic(&self.path, &bytes)
        {
            eprintln!("release cache: {e}");
        }
    }
}

/// Reads GitHub's answer, keeping only downloads that really belong to `repo`'s releases.
fn parse_release(text: &str, repo: &str) -> Result<Release, String> {
    let gh: GhRelease = serde_json::from_str(text).map_err(|e| format!("unexpected answer from GitHub: {e}"))?;
    let assets = gh
        .assets
        .into_iter()
        .map(|a| Asset {
            sha256: a.digest.as_deref().and_then(|d| d.strip_prefix("sha256:")).map(str::to_string),
            name: a.name,
            size: a.size,
            url: a.browser_download_url,
        })
        .collect();
    Ok(Release { tag: gh.tag_name, published_at: gh.published_at.unwrap_or_default(), html_url: gh.html_url, assets }.for_repo(repo))
}

impl Release {
    /// Keeps only downloads from `repo`'s own releases, and only well-formed SHA-256s. Applied to
    /// every release, whether it came from GitHub or from the release index.
    pub fn for_repo(mut self, repo: &str) -> Self {
        let prefix = format!("https://github.com/{repo}/releases/download/").to_ascii_lowercase();
        self.assets.retain(|a| {
            let url = a.url.to_ascii_lowercase();
            url.starts_with(&prefix) && !url.contains("/../") && !url.contains(['?', '#', '\\'])
        });
        for a in &mut self.assets {
            a.sha256 = a.sha256.take().filter(|h| h.len() == 64 && h.bytes().all(|b| b.is_ascii_hexdigit())).map(|h| h.to_ascii_lowercase());
        }
        self
    }

    /// The one file matching `pattern`.
    pub fn pick(&self, pattern: &str) -> Result<&Asset, String> {
        let mut hits = self.assets.iter().filter(|a| glob_match(pattern, &a.name));
        match (hits.next(), hits.next()) {
            (Some(asset), None) => Ok(asset),
            (None, _) => Err(format!("release {} has no file matching {pattern}", self.tag)),
            (Some(_), Some(_)) => Err(format!("more than one file in release {} matches {pattern}", self.tag)),
        }
    }

    /// `2026-10-13T09:12:00Z` → `2026-10-13`.
    pub fn date(&self) -> &str {
        self.published_at.get(..10).unwrap_or(&self.published_at)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_only_this_repos_downloads() {
        let text = r#"{
            "tag_name": "15.2.0", "published_at": "2026-10-13T09:12:00Z",
            "html_url": "https://github.com/BurntSushi/ripgrep/releases/tag/15.2.0",
            "assets": [
                { "name": "ripgrep-15.2.0-x86_64-pc-windows-msvc.zip", "size": 10,
                  "browser_download_url": "https://github.com/BurntSushi/ripgrep/releases/download/15.2.0/ripgrep-15.2.0-x86_64-pc-windows-msvc.zip",
                  "digest": "sha256:3750B2E93F37E0C692657DA574D7019A101C0084DA05A790C83FD335BAD973E4" },
                { "name": "evil.zip", "size": 10,
                  "browser_download_url": "https://github.com/BurntSushi/ripgrep/releases/download/../../../evil/x/releases/download/1/evil.zip" },
                { "name": "elsewhere.zip", "size": 10, "browser_download_url": "https://example.com/elsewhere.zip" }
            ]
        }"#;
        let r = parse_release(text, "burntsushi/ripgrep").unwrap();
        assert_eq!(r.assets.len(), 1);
        assert_eq!(r.assets[0].sha256.as_deref(), Some("3750b2e93f37e0c692657da574d7019a101c0084da05a790c83fd335bad973e4"));
        assert_eq!(r.date(), "2026-10-13");
        assert!(r.pick("ripgrep-*-x86_64-pc-windows-msvc.zip").is_ok());
        assert!(r.pick("ripgrep-*-aarch64-apple-darwin.tar.gz").is_err());
    }
}
