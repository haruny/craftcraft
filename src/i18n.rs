//! The launcher's own text in the user's language: English unless they pick another in Settings.
//!
//! Translations are `locales/<code>.json`, built in: flat objects of keys to text, with `{name}`
//! placeholders. A key a translation lacks falls back to English. Keys for a count end in a
//! plural category (`.one`, `.other`, and for Arabic `.zero`, `.two`, `.few`, `.many`). Text from
//! the catalog (taglines, descriptions, categories, notices) isn't translated.
//!
//! egui lays text out left to right and shapes each font's run on its own, so for Arabic,
//! [`display`] puts a line in visual order before it's drawn.

use std::collections::HashMap;
use std::fmt::Display;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU8, Ordering};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "String", into = "&'static str")]
pub enum Lang {
    #[default]
    En,
    Fr,
    Es,
    Zh,
    Hi,
    Ar,
    Bn,
    Pt,
}

impl Lang {
    pub const ALL: [Lang; 8] = [Lang::En, Lang::Fr, Lang::Es, Lang::Zh, Lang::Hi, Lang::Ar, Lang::Bn, Lang::Pt];

    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Fr => "fr",
            Lang::Es => "es",
            Lang::Zh => "zh",
            Lang::Hi => "hi",
            Lang::Ar => "ar",
            Lang::Bn => "bn",
            Lang::Pt => "pt",
        }
    }

    /// The language's name in itself, for the picker.
    pub fn name(self) -> &'static str {
        match self {
            Lang::En => "English",
            Lang::Fr => "Français",
            Lang::Es => "Español",
            Lang::Zh => "中文（简体）",
            Lang::Hi => "हिन्दी",
            Lang::Ar => "العربية",
            Lang::Bn => "বাংলা",
            Lang::Pt => "Português",
        }
    }

    pub fn rtl(self) -> bool {
        self == Lang::Ar
    }

    /// Written in the Latin alphabet, which the bundled fonts cover. The others need a system font.
    pub fn latin(self) -> bool {
        matches!(self, Lang::En | Lang::Fr | Lang::Es | Lang::Pt)
    }

    fn source(self) -> &'static str {
        match self {
            Lang::En => include_str!("../locales/en.json"),
            Lang::Fr => include_str!("../locales/fr.json"),
            Lang::Es => include_str!("../locales/es.json"),
            Lang::Zh => include_str!("../locales/zh.json"),
            Lang::Hi => include_str!("../locales/hi.json"),
            Lang::Ar => include_str!("../locales/ar.json"),
            Lang::Bn => include_str!("../locales/bn.json"),
            Lang::Pt => include_str!("../locales/pt.json"),
        }
    }

    /// The CLDR plural category for `n` (whole numbers only).
    fn plural(self, n: u64) -> &'static str {
        match self {
            Lang::En | Lang::Es => {
                if n == 1 {
                    "one"
                } else {
                    "other"
                }
            }
            Lang::Fr | Lang::Pt | Lang::Hi | Lang::Bn => {
                if n <= 1 {
                    "one"
                } else {
                    "other"
                }
            }
            Lang::Zh => "other",
            Lang::Ar => match (n, n % 100) {
                (0, _) => "zero",
                (1, _) => "one",
                (2, _) => "two",
                (_, 3..=10) => "few",
                (_, 11..=99) => "many",
                _ => "other",
            },
        }
    }

    fn table(self) -> &'static HashMap<String, String> {
        static TABLES: [OnceLock<HashMap<String, String>>; Lang::ALL.len()] = [const { OnceLock::new() }; Lang::ALL.len()];
        TABLES[self as usize].get_or_init(|| {
            serde_json::from_str(self.source()).unwrap_or_else(|e| {
                eprintln!("locales/{}.json: {e}", self.code());
                HashMap::new()
            })
        })
    }
}

impl From<String> for Lang {
    /// An unknown code (from a newer launcher, say) is English rather than an unreadable state file.
    fn from(code: String) -> Self {
        Lang::ALL.into_iter().find(|l| l.code() == code).unwrap_or_default()
    }
}

impl From<Lang> for &'static str {
    fn from(lang: Lang) -> Self {
        lang.code()
    }
}

static CURRENT: AtomicU8 = AtomicU8::new(0);

pub fn set(lang: Lang) {
    CURRENT.store(lang as u8, Ordering::Relaxed);
}

pub fn current() -> Lang {
    Lang::ALL.get(usize::from(CURRENT.load(Ordering::Relaxed))).copied().unwrap_or_default()
}

/// Placeholder values: `&[("name", &name)]`.
pub type Args<'a> = &'a [(&'a str, &'a dyn Display)];

/// `key` in the current language, as written (not in visual order): for the system (the tray),
/// the clipboard, and [`display`] or wrapping by the caller. Unknown placeholders stay as they are.
pub fn text(key: &str, args: Args) -> String {
    let template = current().table().get(key).or_else(|| Lang::En.table().get(key)).map_or(key, String::as_str);
    fill(template, args)
}

/// [`text`] for a count, `n`, which is also the `{n}` placeholder.
pub fn plural_text(key: &str, n: u64, args: Args) -> String {
    let lang = current();
    let find = |lang: Lang| {
        let table = lang.table();
        table.get(&format!("{key}.{}", lang.plural(n))).or_else(|| table.get(&format!("{key}.other")))
    };
    let template = find(lang).or_else(|| find(Lang::En)).map_or(key, String::as_str);
    let count: &dyn Display = &n;
    let mut all = vec![("n", count)];
    all.extend_from_slice(args);
    fill(template, &all)
}

/// `key`, ready to draw on one line.
pub fn t(key: &str) -> String {
    display(&text(key, &[]))
}

/// [`text`], ready to draw on one line.
pub fn tf(key: &str, args: Args) -> String {
    display(&text(key, args))
}

/// [`plural_text`], ready to draw on one line.
pub fn tn(key: &str, n: u64, args: Args) -> String {
    display(&plural_text(key, n, args))
}

fn fill(template: &str, args: Args) -> String {
    let mut out = template.to_string();
    for (name, value) in args {
        let placeholder = format!("{{{name}}}");
        if out.contains(&placeholder) {
            out = out.replace(&placeholder, &value.to_string());
        }
    }
    out
}

/// Puts each line of `text` in the order egui should draw it: unchanged, unless the language is
/// written right to left and the line is too. (Text from the catalog, in English, isn't.)
pub fn display(text: &str) -> String {
    if !is_rtl(text) {
        return text.to_string();
    }
    text.split('\n').map(|line| if is_rtl(line) { visual(line) } else { line.to_string() }).collect::<Vec<_>>().join("\n")
}

/// Whether `text` is laid out right to left: in a right-to-left language, with its letters in it.
pub fn is_rtl(text: &str) -> bool {
    current().rtl() && text.chars().any(|c| class(c) == Class::Rtl)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Class {
    /// A run of Arabic letters. The shaper joins and reverses it, so it moves as one piece.
    Rtl,
    /// A Latin (or other left-to-right) letter.
    Ltr,
    Number,
    Neutral,
}

fn class(c: char) -> Class {
    if matches!(c as u32, 0x0590..=0x08FF | 0xFB1D..=0xFDFF | 0xFE70..=0xFEFF) {
        Class::Rtl
    } else if c.is_ascii_digit() {
        Class::Number
    } else if c.is_alphabetic() {
        Class::Ltr
    } else {
        Class::Neutral
    }
}

/// One right-to-left line in visual order: a simplified Unicode bidi algorithm (UAX #9) for a
/// right-to-left paragraph without embeddings. Runs of Arabic letters are kept in logical order,
/// since each is a separate font run that the shaper lays out right to left itself.
fn visual(line: &str) -> String {
    // Pieces: whole runs of Arabic letters, and single other characters.
    let mut pieces: Vec<(String, Class)> = Vec::new();
    for c in line.chars() {
        let k = class(c);
        match pieces.last_mut() {
            Some((s, Class::Rtl)) if k == Class::Rtl => s.push(c),
            _ => pieces.push((c.to_string(), k)),
        }
    }
    // A separator between digits ("1.5", "5,000") is part of the number.
    for i in 1..pieces.len().saturating_sub(1) {
        if pieces[i].1 == Class::Neutral && matches!(pieces[i].0.as_str(), "." | "," | ":" | "/") && pieces[i - 1].1 == Class::Number && pieces[i + 1].1 == Class::Number {
            pieces[i].1 = Class::Number;
        }
    }
    // A number after Latin text belongs to it ("ripgrep 14.1"); otherwise it counts as
    // right-to-left for the neutrals around it, though its digits still read left to right.
    let mut last_strong = Class::Rtl;
    for piece in &mut pieces {
        match piece.1 {
            Class::Number if last_strong == Class::Ltr => piece.1 = Class::Ltr,
            Class::Rtl | Class::Ltr => last_strong = piece.1,
            _ => {}
        }
    }
    // Levels: 1 for right to left, 2 for left to right. Neutrals between two left-to-right
    // pieces go with them; any others go with the paragraph, right to left.
    let strong = |k: Class| if k == Class::Ltr { Class::Ltr } else { Class::Rtl };
    let mut levels: Vec<u8> = pieces.iter().map(|(_, k)| if matches!(k, Class::Rtl) { 1 } else { 2 }).collect();
    let mut i = 0;
    while i < pieces.len() {
        if pieces[i].1 != Class::Neutral {
            i += 1;
            continue;
        }
        let start = i;
        while i < pieces.len() && pieces[i].1 == Class::Neutral {
            i += 1;
        }
        let before = start.checked_sub(1).map_or(Class::Rtl, |j| strong(pieces[j].1));
        let after = pieces.get(i).map_or(Class::Rtl, |p| strong(p.1));
        let level = if before == Class::Ltr && after == Class::Ltr { 2 } else { 1 };
        levels[start..i].fill(level);
    }
    // Reverse each left-to-right stretch, then the whole line: left-to-right text ends up in its
    // own order, everything else reversed.
    let mut order: Vec<usize> = (0..pieces.len()).collect();
    let mut i = 0;
    while i < order.len() {
        if levels[i] < 2 {
            i += 1;
            continue;
        }
        let start = i;
        while i < order.len() && levels[i] >= 2 {
            i += 1;
        }
        order[start..i].reverse();
    }
    order.reverse();
    order
        .into_iter()
        .map(|i| {
            let (s, k) = &pieces[i];
            if *k == Class::Neutral && levels[i] == 1 { mirror(s) } else { s.clone() }
        })
        .collect()
}

/// A bracket drawn right to left faces the other way.
fn mirror(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '(' => ')',
            ')' => '(',
            '[' => ']',
            ']' => '[',
            '{' => '}',
            '}' => '{',
            '<' => '>',
            '>' => '<',
            '«' => '»',
            '»' => '«',
            c => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    fn placeholders(s: &str) -> BTreeSet<&str> {
        s.split('{').skip(1).filter_map(|p| p.split_once('}')).map(|(name, _)| name).collect()
    }

    /// A key without its plural category.
    fn base(key: &str) -> &str {
        match key.rsplit_once('.') {
            Some((base, "zero" | "one" | "two" | "few" | "many" | "other")) => base,
            _ => key,
        }
    }

    #[test]
    fn translations_match_english() {
        let en = Lang::En.table();
        assert!(!en.is_empty());
        let en_bases: BTreeSet<&str> = en.keys().map(|k| base(k)).collect();
        for lang in Lang::ALL {
            let table = lang.table();
            assert!(!table.is_empty(), "{} didn't parse", lang.code());
            let bases: BTreeSet<&str> = table.keys().map(|k| base(k)).collect();
            assert_eq!(bases, en_bases, "{} has different keys from en", lang.code());
            for (key, value) in table {
                let english = en.get(key).or_else(|| en.get(&format!("{}.other", base(key)))).unwrap();
                let allowed = placeholders(english);
                // A plural form may leave out {n} ("an hour"), but mustn't invent placeholders.
                let extra: Vec<_> = placeholders(value).difference(&allowed).copied().collect();
                assert!(extra.is_empty(), "{}: {key} has unknown placeholders {extra:?}", lang.code());
                if base(key) == key {
                    assert_eq!(placeholders(value), allowed, "{}: {key} placeholders differ from en", lang.code());
                }
            }
            for key in en.keys().filter(|k| base(k) != k.as_str()) {
                let other = format!("{}.other", base(key));
                assert!(table.contains_key(&other), "{}: missing {other}", lang.code());
            }
        }
    }

    #[test]
    fn plurals() {
        assert_eq!(Lang::En.plural(1), "one");
        assert_eq!(Lang::En.plural(0), "other");
        assert_eq!(Lang::Fr.plural(0), "one");
        assert_eq!(Lang::Zh.plural(1), "other");
        assert_eq!(Lang::Ar.plural(2), "two");
        assert_eq!(Lang::Ar.plural(105), "few");
        assert_eq!(Lang::Ar.plural(11), "many");
        assert_eq!(Lang::Ar.plural(100), "other");
    }

    #[test]
    fn fills_placeholders() {
        assert_eq!(fill("{name} {version} is installed", &[("name", &"ripgrep"), ("version", &"14.1")]), "ripgrep 14.1 is installed");
        assert_eq!(fill("{a} and {missing}", &[("a", &1)]), "1 and {missing}");
    }

    #[test]
    fn unknown_language_is_english() {
        assert_eq!(Lang::from("tlh".to_string()), Lang::En);
        assert_eq!(Lang::from("ar".to_string()), Lang::Ar);
    }

    #[test]
    fn right_to_left_lines() {
        // Arabic words swap places but stay whole: the shaper reverses their letters.
        assert_eq!(visual("مرحبا بك"), "بك مرحبا");
        // Latin text and numbers keep their own order.
        assert_eq!(visual("ثبّت ripgrep 14.1 الآن"), "الآن ripgrep 14.1 ثبّت");
        assert_eq!(visual("تحديث: 1.0 ← 2.0"), "2.0 ← 1.0 :تحديث");
        assert_eq!(visual("CraftCraft 0.2"), "CraftCraft 0.2");
        // Brackets face the other way, and Arabic punctuation stays with its word.
        assert_eq!(visual("ملاحظات (14.1)"), "(14.1) ملاحظات");
        assert_eq!(visual("هل أنت متأكد؟"), "متأكد؟ أنت هل");
        assert_eq!(visual("جارٍ التحقق…"), "…التحقق جارٍ");
    }

    #[test]
    fn english_stays_left_to_right() {
        // Text from the catalog, shown while the launcher is in Arabic.
        assert!(!is_rtl("Well-known software, rewritten in Rust."));
        assert_eq!(display("Well-known software, rewritten in Rust."), "Well-known software, rewritten in Rust.");
    }
}
