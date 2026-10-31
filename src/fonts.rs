//! Fonts for the languages the bundled Geist doesn't cover (Chinese, Hindi, Arabic, Bengali),
//! taken from the system rather than shipped: a CJK font alone would triple the launcher's size.

use std::path::PathBuf;
use std::sync::{Arc, Mutex, PoisonError};

use eframe::egui::FontData;

use crate::i18n::Lang;

/// A font file, and which font in it when it's a collection (`.ttc`).
type Candidate = (&'static str, u32);

/// The first of `lang`'s candidate fonts that's on this computer, with a name for egui's font
/// list (its path, so two languages sharing a file share it). Read once, then kept.
pub fn system(lang: Lang) -> Option<(String, Arc<FontData>)> {
    static LOADED: Mutex<Vec<(Lang, Option<(String, Arc<FontData>)>)>> = Mutex::new(Vec::new());
    if lang.latin() {
        return None;
    }
    let mut loaded = LOADED.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some((_, found)) = loaded.iter().find(|(l, _)| *l == lang) {
        return found.clone();
    }
    let found = find(candidates(lang)).and_then(|(path, index)| {
        let name = path.display().to_string();
        // Another language may have read this file already.
        if let Some(same) = loaded.iter().filter_map(|(_, f)| f.as_ref()).find(|(n, _)| *n == name) {
            return Some(same.clone());
        }
        match std::fs::read(&path) {
            Ok(bytes) => Some((name, Arc::new(FontData::from_blob(Arc::new(bytes), index)))),
            Err(e) => {
                eprintln!("font {name}: {e}");
                None
            }
        }
    });
    crate::debug::log(format_args!("font for {}: {}", lang.code(), found.as_ref().map_or("none found", |(n, _)| n.as_str())));
    loaded.push((lang, found.clone()));
    found
}

#[cfg(windows)]
fn candidates(lang: Lang) -> &'static [Candidate] {
    match lang {
        Lang::Zh => &[("msyh.ttc", 0), ("msyh.ttf", 0), ("simsun.ttc", 0), ("simhei.ttf", 0)],
        Lang::Hi => &[("Nirmala.ttc", 0), ("Nirmala.ttf", 0), ("mangal.ttf", 0)],
        Lang::Bn => &[("Nirmala.ttc", 0), ("Nirmala.ttf", 0), ("vrinda.ttf", 0)],
        Lang::Ar => &[("segoeui.ttf", 0), ("tahoma.ttf", 0), ("arial.ttf", 0)],
        _ => &[],
    }
}

#[cfg(windows)]
fn find(candidates: &[Candidate]) -> Option<(PathBuf, u32)> {
    let fonts = PathBuf::from(std::env::var_os("WINDIR").unwrap_or_else(|| r"C:\Windows".into())).join("Fonts");
    candidates.iter().map(|&(file, index)| (fonts.join(file), index)).find(|(path, _)| path.is_file())
}

#[cfg(target_os = "macos")]
fn candidates(lang: Lang) -> &'static [Candidate] {
    match lang {
        Lang::Zh => &[
            ("/System/Library/Fonts/PingFang.ttc", 0),
            ("/System/Library/Fonts/Hiragino Sans GB.ttc", 0),
            ("/System/Library/Fonts/STHeiti Light.ttc", 0),
            ("/System/Library/Fonts/Supplemental/Arial Unicode.ttf", 0),
        ],
        Lang::Hi => &[
            ("/System/Library/Fonts/Kohinoor.ttc", 0),
            ("/System/Library/Fonts/Supplemental/DevanagariMT.ttc", 0),
            ("/System/Library/Fonts/Supplemental/ITFDevanagari.ttc", 0),
        ],
        Lang::Bn => &[
            ("/System/Library/Fonts/KohinoorBangla.ttc", 0),
            ("/System/Library/Fonts/Supplemental/Bangla Sangam MN.ttc", 0),
            ("/System/Library/Fonts/Supplemental/Bangla MN.ttc", 0),
        ],
        Lang::Ar => &[
            ("/System/Library/Fonts/SFArabic.ttf", 0),
            ("/System/Library/Fonts/GeezaPro.ttc", 0),
            ("/System/Library/Fonts/Supplemental/GeezaPro.ttc", 0),
            ("/System/Library/Fonts/Supplemental/Arial Unicode.ttf", 0),
        ],
        _ => &[],
    }
}

#[cfg(target_os = "macos")]
fn find(candidates: &[Candidate]) -> Option<(PathBuf, u32)> {
    candidates.iter().map(|&(path, index)| (PathBuf::from(path), index)).find(|(path, _)| path.is_file())
}

/// File names, looked for in the usual font folders: distributions put them in different places.
#[cfg(not(any(windows, target_os = "macos")))]
fn candidates(lang: Lang) -> &'static [Candidate] {
    match lang {
        // In Noto's CJK collection, the third font is Simplified Chinese.
        Lang::Zh => &[
            ("NotoSansCJK-Regular.ttc", 2),
            ("NotoSansCJKsc-Regular.otf", 0),
            ("NotoSansSC-Regular.otf", 0),
            ("NotoSansSC-Regular.ttf", 0),
            ("wqy-microhei.ttc", 0),
            ("wqy-zenhei.ttc", 0),
            ("DroidSansFallbackFull.ttf", 0),
        ],
        Lang::Hi => &[
            ("NotoSansDevanagari-Regular.ttf", 0),
            ("NotoSansDevanagari-Regular.otf", 0),
            ("NotoSansDevanagariUI-Regular.ttf", 0),
            ("Lohit-Devanagari.ttf", 0),
        ],
        Lang::Bn => &[("NotoSansBengali-Regular.ttf", 0), ("NotoSansBengali-Regular.otf", 0), ("NotoSansBengaliUI-Regular.ttf", 0), ("Lohit-Bengali.ttf", 0)],
        Lang::Ar => &[
            ("NotoSansArabic-Regular.ttf", 0),
            ("NotoSansArabic-Regular.otf", 0),
            ("NotoSansArabicUI-Regular.ttf", 0),
            ("NotoNaskhArabic-Regular.ttf", 0),
            ("DejaVuSans.ttf", 0),
        ],
        _ => &[],
    }
}

#[cfg(not(any(windows, target_os = "macos")))]
fn find(candidates: &[Candidate]) -> Option<(PathBuf, u32)> {
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let roots = [
        Some(PathBuf::from("/usr/share/fonts")),
        Some(PathBuf::from("/usr/local/share/fonts")),
        home.as_ref().map(|h| h.join(".local/share/fonts")),
        home.map(|h| h.join(".fonts")),
    ];
    let mut found: std::collections::HashMap<String, PathBuf> = std::collections::HashMap::new();
    for root in roots.into_iter().flatten() {
        walk(&root, 5, &mut |path| {
            if let Some(name) = path.file_name().and_then(|n| n.to_str())
                && candidates.iter().any(|(c, _)| *c == name)
            {
                found.entry(name.to_string()).or_insert_with(|| path.to_path_buf());
            }
        });
    }
    candidates.iter().find_map(|&(name, index)| found.get(name).map(|p| (p.clone(), index)))
}

#[cfg(not(any(windows, target_os = "macos")))]
fn walk(dir: &std::path::Path, depth: u32, visit: &mut impl FnMut(&std::path::Path)) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        match entry.file_type() {
            Ok(t) if t.is_dir() && depth > 0 => walk(&path, depth - 1, visit),
            Ok(t) if t.is_file() || t.is_symlink() => visit(&path),
            _ => {}
        }
    }
}
