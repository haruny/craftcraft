//! The catalog: the content config (`catalog/catalog.json`) that the launcher polls and the website shows.
//!
//! It comes from the network, so parsing is forgiving about what it doesn't know (new fields,
//! platforms, kinds and methods are ignored) and strict about what it acts on: ids become folder
//! names and repos become URLs, so an app with a malformed one is dropped, not trusted.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// The catalog format this launcher understands.
pub const SCHEMA: u32 = 1;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Catalog {
    pub schema: u32,
    pub name: String,
    #[serde(default)]
    pub tagline: String,
    #[serde(default)]
    pub updated: String,
    #[serde(default = "default_refresh_hours")]
    pub refresh_hours: u32,
    pub launcher: LauncherInfo,
    #[serde(default)]
    pub categories: Vec<Category>,
    #[serde(default)]
    pub notices: Vec<Notice>,
    #[serde(default)]
    pub apps: Vec<App>,
    /// App ids to feature, in order: the maintainers' picks from featured.json, merged in when
    /// the catalog is published.
    #[serde(default)]
    pub featured: Vec<String>,
}

fn default_refresh_hours() -> u32 {
    6
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LauncherInfo {
    pub version: String,
    pub repo: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notice {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct App {
    pub id: String,
    pub name: String,
    pub replaces: Vec<String>,
    pub category: String,
    #[serde(default)]
    pub kind: Kind,
    pub tagline: String,
    #[serde(default)]
    pub description: String,
    pub repo: String,
    #[serde(default)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub license: String,
    #[serde(default)]
    pub color: String,
    /// A square PNG or JPEG (https).
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub added: String,
    /// Platform key (`windows-x86_64`) to the release file to use there.
    pub assets: BTreeMap<String, AssetSpec>,
    #[serde(default)]
    pub bin: Vec<String>,
    #[serde(default)]
    pub launch: Launch,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    /// Opens a window. The default: most apps are desktop apps.
    #[default]
    Gui,
    /// Runs full-screen in a terminal.
    Tui,
    /// A command to use from a terminal.
    Cli,
    /// A kind added after this launcher was built; treated like `cli`.
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssetSpec {
    Pattern(String),
    Detailed {
        file: String,
        #[serde(default)]
        method: Option<Method>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Method {
    /// Unpacked into the launcher's apps folder.
    Archive,
    /// Handed to the system installer.
    Installer,
    /// A single runnable file.
    Portable,
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Launch {
    #[serde(default)]
    pub windows: Option<String>,
    #[serde(default)]
    pub macos: Option<String>,
    #[serde(default)]
    pub linux: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Os {
    Windows,
    Macos,
    Linux,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arch {
    X86_64,
    Aarch64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Platform {
    pub os: Os,
    pub arch: Arch,
}

impl Os {
    pub fn key(self) -> &'static str {
        match self {
            Os::Windows => "windows",
            Os::Macos => "macos",
            Os::Linux => "linux",
        }
    }
}

impl Arch {
    pub fn key(self) -> &'static str {
        match self {
            Arch::X86_64 => "x86_64",
            Arch::Aarch64 => "aarch64",
        }
    }
}

impl Platform {
    /// The platform this launcher was built for, or `None` on one the catalog has no key for.
    pub fn current() -> Option<Self> {
        let os = if cfg!(windows) {
            Os::Windows
        } else if cfg!(target_os = "macos") {
            Os::Macos
        } else if cfg!(target_os = "linux") {
            Os::Linux
        } else {
            return None;
        };
        let arch = if cfg!(target_arch = "x86_64") {
            Arch::X86_64
        } else if cfg!(target_arch = "aarch64") {
            Arch::Aarch64
        } else {
            return None;
        };
        Some(Self { os, arch })
    }

    pub fn key(self) -> String {
        format!("{}-{}", self.os.key(), self.arch.key())
    }

    /// Keys to try, best first: Arm Windows and Macs also run x86_64 builds, under emulation.
    pub fn candidate_keys(self) -> Vec<String> {
        let mut keys = vec![self.key()];
        if self.arch == Arch::Aarch64 && self.os != Os::Linux {
            keys.push(Platform { arch: Arch::X86_64, ..self }.key());
        }
        keys
    }
}

impl AssetSpec {
    pub fn pattern(&self) -> &str {
        match self {
            AssetSpec::Pattern(p) | AssetSpec::Detailed { file: p, .. } => p,
        }
    }

    /// The install method: as given, or guessed from the file extension.
    pub fn method(&self) -> Option<Method> {
        match self {
            AssetSpec::Detailed { method: Some(m), .. } => (*m != Method::Unknown).then_some(*m),
            _ => guess_method(self.pattern()),
        }
    }
}

pub const ARCHIVE_EXTS: [&str; 5] = [".zip", ".tar.gz", ".tgz", ".tar.xz", ".txz"];

pub fn guess_method(file: &str) -> Option<Method> {
    let f = file.to_ascii_lowercase();
    if ARCHIVE_EXTS.iter().any(|e| f.ends_with(e)) {
        Some(Method::Archive)
    } else if [".msi", ".exe", ".dmg", ".pkg", ".deb", ".rpm"].iter().any(|e| f.ends_with(e)) {
        Some(Method::Installer)
    } else if f.ends_with(".appimage") {
        Some(Method::Portable)
    } else {
        None
    }
}

/// `*` matches any run of characters; everything else matches itself.
pub fn glob_match(pattern: &str, name: &str) -> bool {
    let parts: Vec<&str> = pattern.split('*').collect();
    if parts.len() == 1 {
        return pattern == name;
    }
    let (first, last) = (parts[0], parts[parts.len() - 1]);
    if !name.starts_with(first) || !name[first.len()..].ends_with(last) || name.len() < first.len() + last.len() {
        return false;
    }
    let mut rest = &name[first.len()..name.len() - last.len()];
    for part in &parts[1..parts.len() - 1] {
        match rest.find(part) {
            Some(i) => rest = &rest[i + part.len()..],
            None => return false,
        }
    }
    true
}

impl App {
    /// The asset to use on `platform`, if the app builds for it (natively or under emulation).
    pub fn asset_for(&self, platform: Platform) -> Option<(&AssetSpec, Method)> {
        platform.candidate_keys().iter().find_map(|k| self.assets.get(k)).and_then(|spec| spec.method().map(|m| (spec, m)))
    }

    pub fn launch_path(&self, os: Os) -> Option<&str> {
        match os {
            Os::Windows => self.launch.windows.as_deref(),
            Os::Macos => self.launch.macos.as_deref(),
            Os::Linux => self.launch.linux.as_deref(),
        }
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }

    /// "grep", "Bash and PowerShell", "curl, wget and HTTPie".
    pub fn replaces_text(&self) -> String {
        match self.replaces.as_slice() {
            [] => String::new(),
            [one] => one.clone(),
            [init @ .., last] => format!("{} and {last}", init.join(", ")),
        }
    }

    /// The tile colour as RGB, falling back to a neutral grey.
    pub fn rgb(&self) -> [u8; 3] {
        let hex = self.color.trim_start_matches('#');
        let byte = |i: usize| hex.get(i..i + 2).and_then(|s| u8::from_str_radix(s, 16).ok());
        match (hex.len(), byte(0), byte(2), byte(4)) {
            (6, Some(r), Some(g), Some(b)) => [r, g, b],
            _ => [0x64, 0x74, 0x8b],
        }
    }
}

pub fn valid_id(id: &str) -> bool {
    let b = id.as_bytes();
    (1..=40).contains(&b.len()) && b[0].is_ascii_alphanumeric() && b.iter().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'-')
}

pub fn valid_repo(repo: &str) -> bool {
    let Some((owner, name)) = repo.split_once('/') else { return false };
    let ok = |s: &str, extra: &[u8]| !s.is_empty() && s.len() <= 100 && s.bytes().all(|c| c.is_ascii_alphanumeric() || extra.contains(&c));
    ok(owner, b"-") && ok(name, b"-_.") && name != "." && name != ".."
}

fn valid_bin(bin: &str) -> bool {
    !bin.is_empty() && bin != "." && bin != ".." && bin.bytes().all(|c| c.is_ascii_alphanumeric() || b"._-".contains(&c))
}

/// A parsed catalog plus what was wrong with it.
#[derive(Debug)]
pub struct Parsed {
    pub catalog: Catalog,
    /// Apps that were dropped, and other problems, for the log and `--check-catalog`.
    pub problems: Vec<String>,
}

impl Catalog {
    pub fn parse(text: &str) -> Result<Parsed, String> {
        let mut catalog: Catalog = serde_json::from_str(text).map_err(|e| format!("not a valid catalog: {e}"))?;
        if catalog.schema > SCHEMA {
            return Err(format!("the catalog uses format {}, and this launcher understands up to {SCHEMA}. Update CraftCraft to see it.", catalog.schema));
        }
        catalog.refresh_hours = catalog.refresh_hours.clamp(1, 168);
        let mut problems = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        catalog.apps.retain(|app| {
            let problem = if !valid_id(&app.id) {
                Some("invalid id".to_string())
            } else if !seen.insert(app.id.clone()) {
                Some("duplicate id".to_string())
            } else if !valid_repo(&app.repo) {
                Some(format!("invalid repo {:?}", app.repo))
            } else {
                app.bin.iter().find(|b| !valid_bin(b)).map(|b| format!("invalid bin {b:?}"))
            };
            if let Some(p) = &problem {
                problems.push(format!("{}: {p}; skipped", app.id));
            }
            problem.is_none()
        });
        for app in &catalog.apps {
            if !catalog.categories.iter().any(|c| c.id == app.category) {
                problems.push(format!("{}: unknown category {:?}", app.id, app.category));
            }
            for (key, spec) in &app.assets {
                if spec.method().is_none() {
                    problems.push(format!("{}.{key}: can't tell how to install {:?}", app.id, spec.pattern()));
                } else if spec.method() == Some(Method::Archive) && app.bin.is_empty() {
                    problems.push(format!("{}.{key}: an archive needs a bin entry", app.id));
                }
            }
        }
        catalog.notices.retain(|n| valid_id(&n.id));
        Ok(Parsed { catalog, problems })
    }

    pub fn app(&self, id: &str) -> Option<&App> {
        self.apps.iter().find(|a| a.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r##"{
        "$schema": "./catalog.schema.json",
        "schema": 1, "name": "CraftCraft", "updated": "2026-10-26", "refresh_hours": 0,
        "launcher": { "version": "0.1.0", "repo": "haruny/craftcraft" },
        "categories": [{ "id": "cli", "name": "Command-line tools" }],
        "future_field": true,
        "apps": [
            { "id": "ripgrep", "name": "ripgrep", "replaces": ["grep"], "category": "cli", "kind": "cli",
              "tagline": "t", "repo": "BurntSushi/ripgrep", "color": "#e11d48", "added": "2026-10-26",
              "assets": { "windows-x86_64": "ripgrep-*-x86_64-pc-windows-msvc.zip", "plan9-mips": "x.zip" },
              "bin": ["rg"] },
            { "id": "../evil", "name": "x", "replaces": ["x"], "category": "cli", "kind": "cli",
              "tagline": "t", "repo": "a/b", "assets": {} },
            { "id": "alacritty", "name": "Alacritty", "replaces": ["iTerm2"], "category": "cli", "kind": "hologram",
              "tagline": "t", "repo": "alacritty/alacritty",
              "assets": { "windows-x86_64": { "file": "Alacritty-*-portable.exe", "method": "portable" },
                          "macos-x86_64": "Alacritty-*.dmg" } }
        ]
    }"##;

    #[test]
    fn parses_and_drops_unsafe_apps() {
        let parsed = Catalog::parse(SAMPLE).unwrap();
        let c = parsed.catalog;
        assert_eq!(c.apps.len(), 2);
        assert_eq!(c.refresh_hours, 1);
        assert!(parsed.problems.iter().any(|p| p.contains("../evil")));
        assert_eq!(c.app("alacritty").unwrap().kind, Kind::Unknown);
    }

    #[test]
    fn refuses_newer_schema() {
        let newer = SAMPLE.replace("\"schema\": 1", "\"schema\": 2");
        assert!(Catalog::parse(&newer).unwrap_err().contains("Update CraftCraft"));
    }

    #[test]
    fn published_catalog_parses_cleanly() {
        let parsed = Catalog::parse(include_str!("../catalog/catalog.json")).unwrap();
        assert!(parsed.problems.is_empty(), "{:#?}", parsed.problems);
    }

    #[test]
    fn picks_assets_and_methods() {
        let c = Catalog::parse(SAMPLE).unwrap().catalog;
        let win = Platform { os: Os::Windows, arch: Arch::X86_64 };
        let win_arm = Platform { os: Os::Windows, arch: Arch::Aarch64 };
        let mac_arm = Platform { os: Os::Macos, arch: Arch::Aarch64 };
        let linux = Platform { os: Os::Linux, arch: Arch::X86_64 };
        let rg = c.app("ripgrep").unwrap();
        assert_eq!(rg.asset_for(win).unwrap().1, Method::Archive);
        assert!(rg.asset_for(win_arm).is_some(), "Arm Windows falls back to x86_64");
        assert!(rg.asset_for(linux).is_none());
        let ala = c.app("alacritty").unwrap();
        assert_eq!(ala.asset_for(win).unwrap().1, Method::Portable);
        assert_eq!(ala.asset_for(mac_arm).unwrap().1, Method::Installer);
    }

    #[test]
    fn globs() {
        assert!(glob_match("ripgrep-*-x86_64-pc-windows-msvc.zip", "ripgrep-15.2.0-x86_64-pc-windows-msvc.zip"));
        assert!(!glob_match("ripgrep-*-x86_64-pc-windows-msvc.zip", "ripgrep-15.2.0-x86_64-pc-windows-gnu.zip"));
        assert!(glob_match("Zed-x86_64.exe", "Zed-x86_64.exe"));
        assert!(!glob_match("zellij-x86_64-pc-windows-msvc.zip", "zellij-no-web-x86_64-pc-windows-msvc.zip"));
        assert!(glob_match("a*b*c", "abc"));
        assert!(!glob_match("ab*ba", "aba"));
        assert!(glob_match("*", ""));
    }

    #[test]
    fn ids_and_repos() {
        assert!(valid_id("coreutils"));
        assert!(!valid_id("../x") && !valid_id("") && !valid_id("Caps") && !valid_id("-x"));
        assert!(valid_repo("BurntSushi/ripgrep") && valid_repo("a-b/c.d_e"));
        assert!(!valid_repo("a/b/c") && !valid_repo("a/..") && !valid_repo("/x") && !valid_repo("a/b?x=1"));
    }

    #[test]
    fn replaces_text_reads_naturally() {
        let mut app = Catalog::parse(SAMPLE).unwrap().catalog.apps.remove(0);
        app.replaces = vec!["curl".into(), "wget".into(), "HTTPie".into()];
        assert_eq!(app.replaces_text(), "curl, wget and HTTPie");
    }
}
