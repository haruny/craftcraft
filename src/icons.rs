//! App icons: fetched once, cached on disk by URL, and decoded off the UI thread.

use eframe::egui::ColorImage;

use crate::http::Http;
use crate::paths::{Paths, write_atomic};

const MAX_BYTES: u64 = 2 * 1024 * 1024;
/// Icons are drawn at up to 56 px, so 128 px is plenty even on high-DPI screens.
const SIZE: u32 = 128;

pub fn load(http: &Http, paths: &Paths, url: &str) -> Result<ColorImage, String> {
    if !url.starts_with("https://") {
        return Err("icons must be https".into());
    }
    let dir = paths.cache().join("icons");
    let file = dir.join(format!("{:016x}", fnv1a(url)));
    let bytes = match std::fs::read(&file) {
        Ok(bytes) => bytes,
        Err(_) => {
            let bytes = http.get_bytes(url, MAX_BYTES)?;
            _ = std::fs::create_dir_all(&dir);
            _ = write_atomic(&file, &bytes);
            bytes
        }
    };
    let image = image::load_from_memory(&bytes).map_err(|e| format!("not an image: {e}"))?;
    let image = if image.width() > SIZE || image.height() > SIZE { image.resize(SIZE, SIZE, image::imageops::FilterType::Lanczos3) } else { image };
    let rgba = image.to_rgba8();
    Ok(ColorImage::from_rgba_unmultiplied([rgba.width() as usize, rgba.height() as usize], rgba.as_raw()))
}

/// A stable file name for a URL.
fn fnv1a(s: &str) -> u64 {
    s.bytes().fold(0xcbf2_9ce4_8422_2325, |h, b| (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3))
}
