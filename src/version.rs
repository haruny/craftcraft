//! Comparing release tags. Projects tag releases in their own ways (`v10.5.0`, `15.2.0`,
//! `25.07.1`, `artcraft-v0.41.0`), so this finds the dotted version number in a tag and compares
//! those, and falls back to "different means newer" when a tag has none.

use std::cmp::Ordering;

/// The tag from its version number on: `v1.2.3-rc1` and `artcraft-v1.2.3-rc1` give `1.2.3-rc1`.
fn core(tag: &str) -> &str {
    let t = tag.trim();
    let b = t.as_bytes();
    let starts_word = |i: usize| i == 0 || !b[i - 1].is_ascii_alphanumeric();
    (0..b.len()).find(|&i| b[i].is_ascii_digit() && (starts_word(i) || (matches!(b[i - 1], b'v' | b'V') && starts_word(i - 1)))).map_or(t, |i| &t[i..])
}

/// The version's numbers: `v1.2.3-beta` gives `[1, 2, 3]`. A single number (`nightly-2026-…`)
/// doesn't count as a version.
fn numbers(tag: &str) -> Option<Vec<u64>> {
    let t = core(tag);
    let end = t.find(|c: char| !(c.is_ascii_digit() || c == '.')).unwrap_or(t.len());
    let parts: Vec<u64> = t[..end].split('.').filter(|p| !p.is_empty()).map(|p| p.parse().ok()).collect::<Option<_>>()?;
    (parts.len() >= 2).then_some(parts)
}

fn has_suffix(tag: &str) -> bool {
    core(tag).contains(|c: char| !(c.is_ascii_digit() || c == '.'))
}

/// Whether `latest` is newer than `installed`.
pub fn is_newer(latest: &str, installed: &str) -> bool {
    match (numbers(latest), numbers(installed)) {
        (Some(a), Some(b)) => {
            let len = a.len().max(b.len());
            let pad = |v: &[u64]| (0..len).map(|i| v.get(i).copied().unwrap_or(0)).collect::<Vec<_>>();
            match pad(&a).cmp(&pad(&b)) {
                Ordering::Greater => true,
                Ordering::Less => false,
                // 1.0.0 is newer than 1.0.0-rc1.
                Ordering::Equal => !has_suffix(latest) && has_suffix(installed),
            }
        }
        _ => latest.trim() != installed.trim(),
    }
}

/// `v10.5.0` and `artcraft-v10.5.0` → `10.5.0`, for display.
pub fn display(tag: &str) -> &str {
    if numbers(tag).is_some() { core(tag) } else { tag.trim() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_tags() {
        assert!(is_newer("v10.5.0", "v10.4.2"));
        assert!(is_newer("15.2.0", "14.1.1"));
        assert!(is_newer("0.116.1", "0.99.0"));
        assert!(!is_newer("v1.0.0", "1.0.0"));
        assert!(!is_newer("1.0", "1.0.0"));
        assert!(is_newer("1.0.0", "1.0.0-rc1"));
        assert!(!is_newer("1.0.0-rc1", "1.0.0"));
        assert!(!is_newer("0.9.9", "1.0.0"));
        assert!(is_newer("nightly-2026-10-26", "nightly-2026-10-25"));
        assert!(!is_newer("nightly-2026-10-26", "nightly-2026-10-26"));
        // Prefixed tags against what an installer reports.
        assert!(!is_newer("artcraft-v0.41.0", "0.41.0"));
        assert!(is_newer("artcraft-v0.41.0", "0.40.2"));
        assert!(is_newer("v0.13.5", "0.9.27"));
    }

    #[test]
    fn displays_tags() {
        assert_eq!(display("v10.5.0"), "10.5.0");
        assert_eq!(display("15.2.0"), "15.2.0");
        assert_eq!(display("artcraft-v0.41.0"), "0.41.0");
        assert_eq!(display("x86_64-v2"), "x86_64-v2");
        assert_eq!(display("very-tag"), "very-tag");
    }
}
