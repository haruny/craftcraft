//! Usage statistics, in two layers (see Privacy_Statement.md):
//!
//! - Counts: release builds fetch the catalog through CraftCraft's own endpoint, which counts the
//!   requests by launcher version and type of computer (from the User-Agent) and keeps nothing else.
//! - Usage: launches, and which apps are installed, updated, opened and removed, sent with a random
//!   id so they can be counted per installation. Only once the user has seen the notice about it,
//!   and never when they've turned it off in Settings.
//!
//! The endpoint is built in from `$CRAFTCRAFT_TELEMETRY` (the release workflow sets it). Without
//! it, as in a build from source, there's neither layer. `$DO_NOT_TRACK` or
//! `$CRAFTCRAFT_NO_TELEMETRY` turns both off on a computer.

use std::sync::OnceLock;

use crate::http::Http;
use crate::state::State;

/// Where usage statistics go, and the catalog comes from (the endpoint passes it on from
/// GitHub Pages). `None` turns both off.
pub fn endpoint() -> Option<&'static str> {
    let built_in = option_env!("CRAFTCRAFT_TELEMETRY").map(|e| e.trim().trim_end_matches('/')).filter(|e| acceptable(e));
    built_in.filter(|_| !opted_out_by_environment())
}

/// https, or in debug builds an endpoint on this machine (`wrangler dev`).
fn acceptable(endpoint: &str) -> bool {
    endpoint.starts_with("https://") || (cfg!(debug_assertions) && ["http://localhost:", "http://127.0.0.1:"].iter().any(|p| endpoint.starts_with(p)))
}

/// `DO_NOT_TRACK` (https://consoledonottrack.com) or `CRAFTCRAFT_NO_TELEMETRY`, set to anything but 0.
pub fn opted_out_by_environment() -> bool {
    ["DO_NOT_TRACK", "CRAFTCRAFT_NO_TELEMETRY"].iter().any(|k| std::env::var(k).is_ok_and(|v| !v.trim().is_empty() && v.trim() != "0"))
}

/// Whether the launcher can send usage statistics at all: so whether to ask, and to show the setting.
pub fn available() -> bool {
    endpoint().is_some()
}

/// Whether the user has seen the notice and left usage statistics on.
pub fn allowed(state: &State) -> bool {
    available() && state.settings.share_usage && state.usage.asked && state.usage.id.is_some()
}

/// A new installation id: 128 random bits, unconnected to the computer or the user.
pub fn new_id() -> String {
    use std::hash::{BuildHasher, Hasher};
    // Each RandomState is keyed from the operating system's random source.
    (0..2)
        .map(|_| {
            let mut h = std::collections::hash_map::RandomState::new().build_hasher();
            h.write_u128(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos()));
            format!("{:016x}", h.finish())
        })
        .collect()
}

/// Sends one event in the background, if allowed. `app` is the catalog id and the version
/// involved; `detail` says a little more (which step an install failed at). Failures are ignored.
pub fn record(state: &State, event: &'static str, app: Option<(&str, &str)>, detail: Option<&str>) {
    let (Some(endpoint), Some(id)) = (endpoint(), state.usage.id.clone()) else { return };
    if !allowed(state) {
        return;
    }
    let body = serde_json::json!({
        "id": id,
        "event": event,
        "app": app.map(|a| a.0),
        "app_version": app.map(|a| a.1),
        "detail": detail,
        "launcher": env!("CARGO_PKG_VERSION"),
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "os_version": os_version(),
    });
    let url = format!("{endpoint}/event");
    _ = std::thread::Builder::new().name("usage".into()).spawn(move || {
        if let Err(e) = Http::new().post_json(&url, &body.to_string()) {
            crate::debug::log(format_args!("usage statistics: {e}"));
        }
    });
}

/// Sent with every request: `CraftCraft/0.1.0 (windows; x86_64; 10.0.26300; +https://…)`.
pub fn user_agent() -> &'static str {
    static UA: OnceLock<String> = OnceLock::new();
    UA.get_or_init(|| {
        format!(
            "CraftCraft/{} ({}; {}; {}; +https://github.com/haruny/craftcraft)",
            env!("CARGO_PKG_VERSION"),
            std::env::consts::OS,
            std::env::consts::ARCH,
            os_version()
        )
    })
}

/// The operating system's version (`10.0.26300`, `15.1`, `ubuntu 24.04`), or `unknown`.
pub fn os_version() -> &'static str {
    static VERSION: OnceLock<String> = OnceLock::new();
    VERSION.get_or_init(|| {
        let v = read_os_version().unwrap_or_default();
        let v: String = v.chars().filter(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | ' ')).take(32).collect();
        if v.trim().is_empty() { "unknown".into() } else { v.trim().to_string() }
    })
}

#[cfg(windows)]
fn read_os_version() -> Option<String> {
    use winreg::RegKey;
    use winreg::enums::HKEY_LOCAL_MACHINE;
    let key = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion").ok()?;
    let major: u32 = key.get_value("CurrentMajorVersionNumber").ok()?;
    let minor: u32 = key.get_value("CurrentMinorVersionNumber").unwrap_or(0);
    let build: String = key.get_value("CurrentBuildNumber").ok()?;
    Some(format!("{major}.{minor}.{build}"))
}

#[cfg(target_os = "macos")]
fn read_os_version() -> Option<String> {
    let out = std::process::Command::new("/usr/bin/sw_vers").arg("-productVersion").output().ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}

#[cfg(not(any(windows, target_os = "macos")))]
fn read_os_version() -> Option<String> {
    let text = std::fs::read_to_string("/etc/os-release").ok()?;
    let field = |name: &str| text.lines().find_map(|l| l.strip_prefix(name)?.strip_prefix('=').map(|v| v.trim_matches('"').to_string()));
    Some(format!("{} {}", field("ID")?, field("VERSION_ID").unwrap_or_default()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_random_and_hex() {
        let (a, b) = (new_id(), new_id());
        assert_eq!(a.len(), 32);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
        assert_ne!(a, b);
    }

    #[test]
    fn user_agent_names_the_computer() {
        let ua = user_agent();
        assert!(ua.starts_with(concat!("CraftCraft/", env!("CARGO_PKG_VERSION"), " (")));
        assert!(ua.contains(std::env::consts::ARCH));
    }

    #[test]
    fn only_https_or_this_machine() {
        assert!(acceptable("https://t.example.workers.dev"));
        assert!(!acceptable("http://t.example.workers.dev"));
        assert_eq!(acceptable("http://127.0.0.1:8787"), cfg!(debug_assertions));
        assert!(!acceptable("http://localhost.evil.com"));
    }

    #[test]
    fn nothing_is_sent_until_asked() {
        let mut state = State::default();
        assert!(!allowed(&state));
        state.usage.id = Some(new_id());
        assert!(!allowed(&state));
        state.usage.asked = true;
        state.settings.share_usage = false;
        assert!(!allowed(&state));
    }
}
