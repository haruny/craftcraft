//! HTTP: small text documents (the catalog, GitHub's API) revalidated with ETags, and streamed
//! downloads hashed on the way to disk.

use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;

use sha2::{Digest, Sha256};

const USER_AGENT: &str = concat!("CraftCraft/", env!("CARGO_PKG_VERSION"), " (+https://github.com/haruny/craftcraft)");

/// Text responses larger than this are refused.
const TEXT_LIMIT: u64 = 4 * 1024 * 1024;

pub enum Fetched {
    Body {
        text: String,
        etag: Option<String>,
    },
    NotModified,
    NotFound,
    /// GitHub's request limit; `reset` is when it lifts (Unix seconds).
    RateLimited {
        reset: Option<i64>,
    },
}

#[derive(Clone)]
pub struct Http {
    agent: ureq::Agent,
}

impl Http {
    pub fn new() -> Self {
        let agent = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .user_agent(USER_AGENT)
            .timeout_connect(Some(Duration::from_secs(15)))
            .timeout_recv_response(Some(Duration::from_secs(30)))
            .timeout_per_read(Some(Duration::from_secs(60)))
            .build()
            .into();
        Self { agent }
    }

    /// GETs a small text document, sending `etag` as `If-None-Match`.
    pub fn get_text(&self, url: &str, etag: Option<&str>) -> Result<Fetched, String> {
        let mut req = self.agent.get(url);
        if url.starts_with("https://api.github.com/") {
            req = req.header("Accept", "application/vnd.github+json").header("X-GitHub-Api-Version", "2022-11-28");
            if let Some(token) = github_token() {
                req = req.header("Authorization", &format!("Bearer {token}"));
            }
        }
        if let Some(etag) = etag {
            req = req.header("If-None-Match", etag);
        }
        let mut resp = req.call().map_err(|e| e.to_string())?;
        let status = resp.status().as_u16();
        let header = |name: &str| resp.headers().get(name).and_then(|v| v.to_str().ok()).map(str::to_string);
        match status {
            200 => {
                let etag = header("etag");
                let text = resp.body_mut().with_config().limit(TEXT_LIMIT).read_to_string().map_err(|e| e.to_string())?;
                Ok(Fetched::Body { text, etag })
            }
            304 => Ok(Fetched::NotModified),
            404 => Ok(Fetched::NotFound),
            403 | 429 if header("x-ratelimit-remaining").as_deref() == Some("0") || status == 429 => {
                Ok(Fetched::RateLimited { reset: header("x-ratelimit-reset").and_then(|r| r.parse().ok()) })
            }
            _ => Err(format!("{url} answered HTTP {status}")),
        }
    }

    /// GETs a small binary file (an icon), refusing anything over `limit` bytes.
    pub fn get_bytes(&self, url: &str, limit: u64) -> Result<Vec<u8>, String> {
        let mut resp = self.agent.get(url).call().map_err(|e| e.to_string())?;
        match resp.status().as_u16() {
            200 => resp.body_mut().with_config().limit(limit).read_to_vec().map_err(|e| e.to_string()),
            status => Err(format!("{url} answered HTTP {status}")),
        }
    }

    /// Streams `url` into `dest`, calling `progress(done, total)` as it goes, and returns the
    /// file's SHA-256 (lowercase hex).
    pub fn download(&self, url: &str, dest: &Path, mut progress: impl FnMut(u64, Option<u64>)) -> Result<String, String> {
        let resp = self.agent.get(url).call().map_err(|e| e.to_string())?;
        let status = resp.status().as_u16();
        if status != 200 {
            return Err(format!("download failed: HTTP {status}"));
        }
        let total = resp.headers().get("content-length").and_then(|v| v.to_str().ok()).and_then(|v| v.parse().ok());
        let mut reader = resp.into_body().into_reader();
        let mut file = std::fs::File::create(dest).map_err(|e| format!("{}: {e}", dest.display()))?;
        let mut hasher = Sha256::new();
        let mut buf = vec![0u8; 256 * 1024];
        let mut done = 0u64;
        loop {
            let n = reader.read(&mut buf).map_err(|e| format!("download interrupted: {e}"))?;
            if n == 0 {
                break;
            }
            hasher.update(&buf[..n]);
            file.write_all(&buf[..n]).map_err(|e| format!("{}: {e}", dest.display()))?;
            done += n as u64;
            progress(done, total);
        }
        file.sync_all().map_err(|e| e.to_string())?;
        if total.is_some_and(|t| t != done) {
            return Err("download ended early".into());
        }
        Ok(to_hex(&hasher.finalize()))
    }
}

pub fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// A GitHub token raises the API limit from 60 to 5,000 requests an hour. Optional.
fn github_token() -> Option<String> {
    ["CRAFTCRAFT_GITHUB_TOKEN", "GITHUB_TOKEN"].iter().find_map(|k| std::env::var(k).ok().filter(|t| !t.trim().is_empty()))
}
