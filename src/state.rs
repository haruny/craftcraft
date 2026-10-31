//! What the launcher remembers between runs: what it installed, which catalog apps the user has
//! already seen (so it can say what's new), dismissed notices, usage statistics, and settings.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::catalog::{Catalog, Method};
use crate::i18n::{self, Lang};
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
    pub usage: Usage,
    pub settings: Settings,
}

/// Usage statistics (see `telemetry`).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Usage {
    /// The user has seen the notice about usage statistics (and answered it).
    pub asked: bool,
    /// This installation's random id. Made when the user agrees, and forgotten when they turn
    /// usage statistics off, so turning them back on starts afresh.
    pub id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// Check the catalog and releases automatically.
    pub auto_check: bool,
    /// How often, when checking automatically: 1 to [`MAX_CHECK_HOURS`].
    pub check_hours: u32,
    /// Windows: closing the window leaves the launcher running in the tray.
    pub close_to_tray: bool,
    pub sort: Sort,
    /// Keep a log, installers and their logs (see `debug`).
    pub debug: bool,
    /// Send anonymous usage statistics, once the user has been told about them.
    pub share_usage: bool,
    /// The language of the launcher's own text.
    pub language: Lang,
}

pub const MAX_CHECK_HOURS: u32 = 168;

impl Default for Settings {
    fn default() -> Self {
        Self { auto_check: true, check_hours: 8, close_to_tray: true, sort: Sort::Featured, debug: false, share_usage: true, language: Lang::En }
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

    pub fn label(self) -> String {
        i18n::t(match self {
            Sort::Featured => "sort.featured",
            Sort::Popular => "sort.popular",
            Sort::Name => "sort.name",
            Sort::Updated => "sort.updated",
        })
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
