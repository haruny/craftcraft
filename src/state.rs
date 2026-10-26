//! What the launcher remembers between runs: what it installed, which catalog apps the user has
//! already seen (so it can say what's new), dismissed notices, and settings.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::catalog::{Catalog, Method};
use crate::paths::{Paths, write_atomic};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct State {
    pub installed: BTreeMap<String, Installed>,
    /// App ids the user has been shown. Empty until the first catalog arrives.
    pub seen_apps: BTreeSet<String>,
    /// Whether a catalog has ever been seen: the first one marks everything as seen, so a new
    /// user isn't told that every app is new.
    pub seen_first_catalog: bool,
    pub dismissed_notices: BTreeSet<String>,
    /// Unix seconds of the last successful catalog check.
    pub last_check: i64,
    pub settings: Settings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// Check the catalog and releases automatically.
    pub auto_check: bool,
    pub sort: Sort,
}

impl Default for Settings {
    fn default() -> Self {
        Self { auto_check: true, sort: Sort::Featured }
    }
}

/// The order apps are listed in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sort {
    /// The catalog's own order.
    #[default]
    Featured,
    /// Most GitHub stars first.
    Popular,
    Name,
    /// Newest release first.
    Updated,
}

impl Sort {
    pub const ALL: [Sort; 4] = [Sort::Featured, Sort::Popular, Sort::Name, Sort::Updated];

    pub fn label(self) -> &'static str {
        match self {
            Sort::Featured => "Featured",
            Sort::Popular => "Most popular",
            Sort::Name => "Name",
            Sort::Updated => "Recently updated",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Installed {
    /// The release tag installed.
    pub tag: String,
    pub method: Method,
    /// For archives, the app's folder; for portable apps, the file; for installers, the
    /// downloaded installer (which may since have been cleaned up).
    pub path: PathBuf,
    /// Executables, absolute, first one is what Open runs.
    #[serde(default)]
    pub bins: Vec<PathBuf>,
    /// Whether the download matched a published SHA-256.
    pub verified: bool,
    pub installed_at: i64,
}

impl State {
    pub fn load(paths: &Paths) -> Self {
        std::fs::read(paths.state_file()).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default()
    }

    pub fn save(&self, paths: &Paths) {
        match serde_json::to_vec_pretty(self) {
            Ok(bytes) => {
                if let Err(e) = write_atomic(&paths.state_file(), &bytes) {
                    eprintln!("saving state: {e}");
                }
            }
            Err(e) => eprintln!("saving state: {e}"),
        }
    }

    /// Ids in `catalog` the user hasn't been shown yet.
    pub fn new_apps(&self, catalog: &Catalog) -> Vec<String> {
        if !self.seen_first_catalog {
            return Vec::new();
        }
        catalog.apps.iter().filter(|a| !self.seen_apps.contains(&a.id)).map(|a| a.id.clone()).collect()
    }

    pub fn mark_all_seen(&mut self, catalog: &Catalog) {
        self.seen_apps.extend(catalog.apps.iter().map(|a| a.id.clone()));
        self.seen_first_catalog = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalog(ids: &[&str]) -> Catalog {
        let apps: Vec<String> = ids
            .iter()
            .map(|id| format!(r#"{{"id":"{id}","name":"{id}","replaces":["x"],"category":"cli","kind":"cli","tagline":"t","repo":"o/{id}","assets":{{}}}}"#))
            .collect();
        let text = format!(r#"{{"schema":1,"name":"c","launcher":{{"version":"0.1.0","repo":"o/l"}},"apps":[{}]}}"#, apps.join(","));
        Catalog::parse(&text).unwrap().catalog
    }

    #[test]
    fn first_catalog_is_not_all_new() {
        let mut state = State::default();
        let first = catalog(&["ripgrep", "fd"]);
        assert!(state.new_apps(&first).is_empty());
        state.mark_all_seen(&first);
        let second = catalog(&["ripgrep", "fd", "bat"]);
        assert_eq!(state.new_apps(&second), ["bat"]);
        state.mark_all_seen(&second);
        assert!(state.new_apps(&second).is_empty());
    }
}
