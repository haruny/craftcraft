//! Where the launcher keeps things.
//!
//! Everything lives in one data folder (`%LOCALAPPDATA%\CraftCraft\CraftCraft\data` on Windows,
//! `~/Library/Application Support/io.CraftCraft.CraftCraft` on macOS, `~/.local/share/craftcraft`
//! on Linux), or in `$CRAFTCRAFT_HOME` when that is set.

use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Paths {
    pub root: PathBuf,
}

impl Paths {
    pub fn new() -> Result<Self, String> {
        let root = match std::env::var_os("CRAFTCRAFT_HOME") {
            Some(home) if !home.is_empty() => PathBuf::from(home),
            _ => directories::ProjectDirs::from("io", "CraftCraft", "CraftCraft").ok_or("couldn't find a home folder")?.data_local_dir().to_path_buf(),
        };
        let paths = Self { root };
        for dir in [paths.apps(), paths.bin(), paths.downloads(), paths.cache(), paths.logs()] {
            std::fs::create_dir_all(&dir).map_err(|e| format!("couldn't create {}: {e}", dir.display()))?;
        }
        Ok(paths)
    }

    /// Unpacked apps, one folder each.
    pub fn apps(&self) -> PathBuf {
        self.root.join("apps")
    }

    pub fn app_dir(&self, id: &str) -> PathBuf {
        self.apps().join(id)
    }

    /// Shims for command-line apps; users can put this on their PATH.
    pub fn bin(&self) -> PathBuf {
        self.root.join("bin")
    }

    pub fn downloads(&self) -> PathBuf {
        self.root.join("downloads")
    }

    pub fn cache(&self) -> PathBuf {
        self.root.join("cache")
    }

    /// System installers' logs, and the launcher's own in debug mode.
    pub fn logs(&self) -> PathBuf {
        self.root.join("logs")
    }

    pub fn state_file(&self) -> PathBuf {
        self.root.join("state.json")
    }
}

/// Writes `bytes` to `path` through a temporary file, so a crash never leaves half a file.
pub fn write_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let tmp = path.with_extension("tmp");
    {
        let mut f = std::fs::File::create(&tmp)?;
        f.write_all(bytes)?;
        f.sync_all()?;
    }
    std::fs::rename(&tmp, path)
}

/// Expands `%VAR%` (Windows) and a leading `~` in a path from the catalog.
pub fn expand(path: &str) -> PathBuf {
    let mut out = String::new();
    let mut rest = path;
    while let Some(start) = rest.find('%') {
        let Some(len) = rest[start + 1..].find('%') else { break };
        let name = &rest[start + 1..start + 1 + len];
        out.push_str(&rest[..start]);
        match std::env::var(name) {
            Ok(value) if !name.is_empty() => out.push_str(&value),
            _ => out.push_str(&rest[start..start + len + 2]),
        }
        rest = &rest[start + len + 2..];
    }
    out.push_str(rest);
    if let Some(tail) = out.strip_prefix("~/").or_else(|| out.strip_prefix("~\\"))
        && let Some(home) = directories::BaseDirs::new()
    {
        return home.home_dir().join(tail);
    }
    PathBuf::from(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_variables() {
        // SAFETY: tests in this module don't read this variable concurrently.
        unsafe { std::env::set_var("CRAFTCRAFT_TEST_DIR", "C:\\Apps") };
        assert_eq!(expand("%CRAFTCRAFT_TEST_DIR%\\Zed\\Zed.exe"), PathBuf::from("C:\\Apps\\Zed\\Zed.exe"));
        assert_eq!(expand("%NOT_SET_ANYWHERE_42%\\x"), PathBuf::from("%NOT_SET_ANYWHERE_42%\\x"));
        assert_eq!(expand("/Applications/Zed.app"), PathBuf::from("/Applications/Zed.app"));
        assert!(expand("~/x").ends_with("x") && !expand("~/x").starts_with("~"));
    }
}
