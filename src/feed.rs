//! Fetching the hosted catalog.
//!
//! The launcher asks for the catalog at start-up and then every few hours (a setting). It sends the
//! last ETag, so an unchanged catalog costs one tiny request, and it keeps the last good copy so
//! it starts instantly and still works offline.

use crate::catalog::{Catalog, Parsed};
use crate::http::{Fetched, Http};
use crate::paths::{Paths, write_atomic};

/// The published catalog. Release builds always use it; users can't change it.
pub const FEED: &str = "https://haruny.github.io/craftcraft/catalog.json";

pub enum FeedResult {
    Updated(Box<Parsed>),
    Unchanged,
    Failed(String),
}

/// The catalog to read. Debug builds also honour `$CRAFTCRAFT_FEED` (an address or a local file),
/// for testing catalog edits before they're published. Builds with a usage statistics endpoint
/// fetch the published catalog through it (see `telemetry`), falling back to [`FEED`].
pub fn source() -> String {
    if cfg!(debug_assertions)
        && let Some(dev) = std::env::var("CRAFTCRAFT_FEED").ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
    {
        return dev;
    }
    match crate::telemetry::endpoint() {
        Some(endpoint) => format!("{endpoint}/catalog.json"),
        None => FEED.to_string(),
    }
}

/// The last catalog fetched, if any.
pub fn cached(paths: &Paths) -> Option<Catalog> {
    let text = std::fs::read_to_string(paths.cache().join("catalog.json")).ok()?;
    Catalog::parse(&text).ok().map(|p| p.catalog)
}

pub fn fetch(http: &Http, paths: &Paths, source: &str) -> FeedResult {
    match fetch_inner(http, paths, source) {
        Ok(result) => result,
        Err(e) => FeedResult::Failed(e),
    }
}

fn fetch_inner(http: &Http, paths: &Paths, source: &str) -> Result<FeedResult, String> {
    let cache = paths.cache().join("catalog.json");
    let etag_file = paths.cache().join("catalog.etag");

    let text = if is_remote(source)? {
        let etag = if cache.exists() { std::fs::read_to_string(&etag_file).ok() } else { None };
        match http.get_text(source, etag.as_deref())? {
            Fetched::Body { text, etag } => {
                let parsed = Catalog::parse(&text)?;
                write_atomic(&cache, text.as_bytes()).map_err(|e| e.to_string())?;
                match etag {
                    Some(etag) => write_atomic(&etag_file, etag.as_bytes()).map_err(|e| e.to_string())?,
                    None => _ = std::fs::remove_file(&etag_file),
                }
                return Ok(FeedResult::Updated(Box::new(parsed)));
            }
            Fetched::NotModified => return Ok(FeedResult::Unchanged),
            Fetched::NotFound => return Err(format!("no catalog at {source}")),
            Fetched::RateLimited { .. } => return Err("the catalog host is limiting requests; trying again later".into()),
        }
    } else {
        let path = source.strip_prefix("file://").unwrap_or(source);
        std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?
    };
    let mut parsed = Catalog::parse(&text)?;
    // Publishing merges in featured.json; do the same for a local catalog, to preview it.
    let featured = crate::index::source(source).replace("releases.json", "featured.json");
    if parsed.catalog.featured.is_empty()
        && let Ok(text) = std::fs::read_to_string(featured.strip_prefix("file://").unwrap_or(&featured))
        && let Ok(f) = serde_json::from_str::<Featured>(&text)
    {
        parsed.catalog.featured = f.featured;
    }
    // A local catalog isn't cached: the cache is the last good copy of the published one.
    Ok(FeedResult::Updated(Box::new(parsed)))
}

#[derive(serde::Deserialize)]
struct Featured {
    #[serde(default)]
    featured: Vec<String>,
}

/// Remote catalogs must use https, except on this machine (a catalog served locally for testing).
fn is_remote(source: &str) -> Result<bool, String> {
    if source.starts_with("https://") {
        return Ok(true);
    }
    if let Some(rest) = source.strip_prefix("http://") {
        let host = rest.split(['/', ':']).next().unwrap_or_default();
        let loopback = matches!(host, "localhost" | "127.0.0.1") || rest.starts_with("[::1]");
        return if loopback { Ok(true) } else { Err("the catalog address must start with https://".into()) };
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_https_or_loopback() {
        assert_eq!(is_remote("https://example.com/catalog.json"), Ok(true));
        assert_eq!(is_remote("http://127.0.0.1:8000/catalog.json"), Ok(true));
        assert_eq!(is_remote("http://localhost/catalog.json"), Ok(true));
        assert!(is_remote("http://example.com/catalog.json").is_err());
        assert!(is_remote("http://[2001:db8::1]/catalog.json").is_err());
        assert!(is_remote("http://localhost.evil.com/catalog.json").is_err());
        assert_eq!(is_remote("D:\\site\\catalog.json"), Ok(false));
    }
}
