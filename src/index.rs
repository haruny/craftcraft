//! The release index: every catalog app's latest release and GitHub stars, in one file.
//!
//! The catalog's workflow rebuilds `releases.json` every hour and publishes it next to
//! `catalog.json`. A launcher fetches that one file (revalidated with an ETag, and cached like the
//! catalog) instead of asking GitHub's API about each app, which allows only 60 requests an hour
//! without a token. So checking for updates costs one request whatever the size of the catalog.
//! An app the index doesn't cover yet (a catalog entry being tested, say) is looked up on GitHub
//! directly, as before.

use std::collections::HashMap;

use serde::Deserialize;

use crate::github::Release;
use crate::http::{Fetched, Http};
use crate::paths::{Paths, write_atomic};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Entry {
    #[serde(default)]
    pub stars: u64,
    /// The SPDX id GitHub detected, for apps whose catalog entry doesn't give a license.
    #[serde(default)]
    pub license: Option<String>,
    /// The latest release; `None` when the project hasn't published one.
    #[serde(default)]
    pub release: Option<Release>,
}

/// By app id.
pub type Index = HashMap<String, Entry>;

#[derive(Deserialize)]
struct File {
    #[serde(default)]
    apps: Index,
}

fn parse(text: &str) -> Option<Index> {
    serde_json::from_str::<File>(text).ok().map(|f| f.apps)
}

/// `releases.json` beside the catalog: an address or a local file.
pub fn source(catalog: &str) -> String {
    let dir = catalog.rfind(['/', '\\']).map_or("", |i| &catalog[..=i]);
    format!("{dir}releases.json")
}

pub fn cached(paths: &Paths) -> Index {
    std::fs::read_to_string(paths.cache().join("releases-index.json")).ok().and_then(|t| parse(&t)).unwrap_or_default()
}

/// A newer index, or `None` when it's unchanged or couldn't be had (the cached one still stands).
pub fn fetch(http: &Http, paths: &Paths, source: &str) -> Option<Index> {
    if !source.starts_with("https://") && !source.starts_with("http://") {
        // Beside a local catalog being tested, if someone generated one.
        return std::fs::read_to_string(source.strip_prefix("file://").unwrap_or(source)).ok().and_then(|t| parse(&t));
    }
    let cache = paths.cache().join("releases-index.json");
    let etag_file = paths.cache().join("releases-index.etag");
    let etag = if cache.exists() { std::fs::read_to_string(&etag_file).ok() } else { None };
    match http.get_text(source, etag.as_deref()) {
        Ok(Fetched::Body { text, etag }) => {
            let index = parse(&text)?;
            _ = write_atomic(&cache, text.as_bytes());
            match etag {
                Some(etag) => _ = write_atomic(&etag_file, etag.as_bytes()),
                None => _ = std::fs::remove_file(&etag_file),
            }
            Some(index)
        }
        Ok(_) => None,
        Err(e) => {
            eprintln!("release index: {e}");
            None
        }
    }
}

/// `73329` → `73.3k`, as GitHub shows star counts.
pub fn short(n: u64) -> String {
    match n {
        0..1_000 => n.to_string(),
        1_000..100_000 => format!("{:.1}k", n as f64 / 1_000.0).replace(".0k", "k"),
        100_000..1_000_000 => format!("{}k", n / 1_000),
        _ => format!("{:.1}M", n as f64 / 1_000_000.0).replace(".0M", "M"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_index_beside_the_catalog() {
        assert_eq!(source("https://haruny.github.io/craftcraft/catalog.json"), "https://haruny.github.io/craftcraft/releases.json");
        assert_eq!(source("catalog\\catalog.json"), "catalog\\releases.json");
        assert_eq!(source("catalog.json"), "releases.json");
    }

    #[test]
    fn shortens_like_github() {
        assert_eq!(short(950), "950");
        assert_eq!(short(1_000), "1k");
        assert_eq!(short(73_329), "73.3k");
        assert_eq!(short(250_400), "250k");
        assert_eq!(short(1_200_000), "1.2M");
    }

    #[test]
    fn reads_the_published_index() {
        let text = r#"{"generated":"x","apps":{
            "a":{"stars":5,"release":{"tag":"v1.0.0","published_at":"2026-10-25T10:00:00Z","html_url":"https://github.com/o/a/releases/tag/v1.0.0",
                 "assets":[{"name":"a.msi","size":3,"url":"https://github.com/o/a/releases/download/v1.0.0/a.msi","sha256":null}]}},
            "b":{"stars":2,"release":null}}}"#;
        let index = parse(text).unwrap();
        assert_eq!(index["a"].stars, 5);
        assert_eq!(index["a"].release.as_ref().unwrap().date(), "2026-10-25");
        assert!(index["b"].release.is_none());
    }
}
