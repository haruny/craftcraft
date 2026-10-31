//! Debug mode, switched on in Settings. The launcher then writes what it does to
//! `logs/craftcraft.log` in the data folder, and keeps the installers it downloads and their logs,
//! which it otherwise deletes once they've run.

use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

static ON: AtomicBool = AtomicBool::new(false);
static LOG: OnceLock<Mutex<PathBuf>> = OnceLock::new();
/// Past this, the log starts again (keeping the one before), so debug mode can be left on.
const MAX_LOG: u64 = 4 << 20;

/// Starts the log in `logs`, on or off as the settings say.
pub fn init(logs: &Path, on: bool) {
    _ = LOG.set(Mutex::new(logs.join("craftcraft.log")));
    set(on);
}

pub fn enabled() -> bool {
    ON.load(Ordering::Relaxed)
}

pub fn set(on: bool) {
    let was = ON.swap(on, Ordering::Relaxed);
    if on && !was {
        log(format_args!("debug mode on: CraftCraft {} on {}-{}", env!("CARGO_PKG_VERSION"), std::env::consts::OS, std::env::consts::ARCH));
    }
}

/// Adds a line to the log in debug mode. It takes `format_args!`, so nothing is formatted otherwise.
pub fn log(line: fmt::Arguments<'_>) {
    if !enabled() {
        return;
    }
    let Some(Ok(path)) = LOG.get().map(Mutex::lock) else { return };
    if fs::metadata(&*path).is_ok_and(|m| m.len() > MAX_LOG) {
        _ = fs::rename(&*path, path.with_extension("old.log"));
    }
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&*path) {
        _ = file.write_all(format!("{} {line}\n", timestamp(crate::now())).as_bytes());
    }
}

/// Unix seconds as `2026-10-31 04:13:49Z`.
fn timestamp(secs: i64) -> String {
    let (days, time) = (secs.div_euclid(86_400), secs.rem_euclid(86_400));
    // The civil date from days since 1970-01-01, after Howard Hinnant's `civil_from_days`.
    let z = days + 719_468;
    let (era, doe) = (z.div_euclid(146_097), z.rem_euclid(146_097));
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02} {:02}:{:02}:{:02}Z", time / 3600, time / 60 % 60, time % 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stamps_times() {
        assert_eq!(timestamp(0), "1970-01-01 00:00:00Z");
        assert_eq!(timestamp(951_782_400), "2000-02-29 00:00:00Z");
        assert_eq!(timestamp(1_000_000_000), "2001-09-09 01:46:40Z");
        assert_eq!(timestamp(1_793_419_147), "2026-10-31 03:59:07Z");
    }
}
