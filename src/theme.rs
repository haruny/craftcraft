//! Colours, type and widget styling, matched to the website. Dark and light follow the system.

use std::sync::Arc;

use eframe::egui::{self, Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Stroke, TextStyle, Theme, Vec2};

/// One theme's colours.
#[derive(Clone, Copy)]
pub struct Palette {
    pub canvas: Color32,
    pub surface: Color32,
    pub surface2: Color32,
    pub hover: Color32,
    pub line: Color32,
    pub line_strong: Color32,
    pub ink: Color32,
    pub muted: Color32,
    pub subtle: Color32,
    pub accent: Color32,
    pub action: Color32,
    pub good: Color32,
}

const fn hex(rgb: u32) -> Color32 {
    Color32::from_rgb((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8)
}

pub const DARK: Palette = Palette {
    canvas: hex(0x0b0c0f),
    surface: hex(0x121419),
    surface2: hex(0x181b21),
    hover: hex(0x1c1f26),
    line: hex(0x23262e),
    line_strong: hex(0x30343e),
    ink: hex(0xf3f4f6),
    muted: hex(0xa8adb8),
    subtle: hex(0x747b88),
    accent: hex(0xff8a50),
    action: hex(0xf0662a),
    good: hex(0x4ade80),
};

pub const LIGHT: Palette = Palette {
    canvas: hex(0xfbfaf8),
    surface: hex(0xffffff),
    surface2: hex(0xf4f2ee),
    hover: hex(0xf7f5f2),
    line: hex(0xe8e5df),
    line_strong: hex(0xd9d5cd),
    ink: hex(0x121317),
    muted: hex(0x4b505b),
    subtle: hex(0x7a7f8a),
    accent: hex(0xc2470f),
    action: hex(0xe65a1c),
    good: hex(0x15803d),
};

pub fn palette(ctx: &egui::Context) -> Palette {
    if ctx.theme() == Theme::Dark { DARK } else { LIGHT }
}

/// Medium-weight text: `RichText::new(..).family(theme::strong())`.
pub fn strong() -> FontFamily {
    FontFamily::Name("strong".into())
}

pub fn install(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    let mut add = |name: &str, bytes: &'static [u8]| {
        fonts.font_data.insert(name.into(), Arc::new(FontData::from_static(bytes)));
    };
    add("geist", include_bytes!("../assets/fonts/Geist-Regular.ttf"));
    add("geist-semibold", include_bytes!("../assets/fonts/Geist-SemiBold.ttf"));
    add("geist-mono", include_bytes!("../assets/fonts/GeistMono-Medium.ttf"));
    // egui's own fonts stay behind Geist for symbols and emoji Geist doesn't have.
    let fallbacks = fonts.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
    fonts.families.entry(FontFamily::Proportional).or_default().insert(0, "geist".into());
    fonts.families.entry(FontFamily::Monospace).or_default().insert(0, "geist-mono".into());
    fonts.families.insert(strong(), std::iter::once("geist-semibold".to_string()).chain(fallbacks).collect());
    ctx.set_fonts(fonts);

    for (theme, p) in [(Theme::Dark, DARK), (Theme::Light, LIGHT)] {
        ctx.style_mut_of(theme, |style| {
            style.text_styles = [
                (TextStyle::Heading, FontId::new(24.0, strong())),
                (TextStyle::Body, FontId::new(14.0, FontFamily::Proportional)),
                (TextStyle::Button, FontId::new(14.0, FontFamily::Proportional)),
                (TextStyle::Small, FontId::new(12.0, FontFamily::Proportional)),
                (TextStyle::Monospace, FontId::new(12.5, FontFamily::Monospace)),
            ]
            .into();
            style.spacing.button_padding = Vec2::new(12.0, 6.0);
            style.spacing.item_spacing = Vec2::new(8.0, 8.0);
            style.spacing.interact_size.y = 30.0;

            let v = &mut style.visuals;
            v.panel_fill = p.canvas;
            v.window_fill = p.surface;
            v.window_stroke = Stroke::new(1.0, p.line_strong);
            v.window_corner_radius = CornerRadius::same(14);
            v.menu_corner_radius = CornerRadius::same(10);
            v.extreme_bg_color = p.surface;
            v.faint_bg_color = p.surface2;
            v.code_bg_color = p.surface2;
            v.weak_text_color = Some(p.subtle);
            v.hyperlink_color = p.accent;
            v.selection.bg_fill = p.action;
            v.selection.stroke = Stroke::new(1.0, Color32::WHITE);
            v.text_cursor.stroke = Stroke::new(2.0, p.action);

            let radius = CornerRadius::same(9);
            v.widgets.noninteractive.bg_fill = p.surface;
            v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, p.line);
            v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, p.ink);
            v.widgets.noninteractive.corner_radius = radius;
            for (w, fill, stroke) in [
                (&mut v.widgets.inactive, p.surface2, p.line_strong),
                (&mut v.widgets.hovered, p.hover, p.subtle),
                (&mut v.widgets.active, p.surface2, p.subtle),
                (&mut v.widgets.open, p.surface2, p.line_strong),
            ] {
                w.bg_fill = fill;
                w.weak_bg_fill = fill;
                w.bg_stroke = Stroke::new(1.0, stroke);
                w.fg_stroke = Stroke::new(1.0, p.ink);
                w.corner_radius = radius;
                w.expansion = 0.0;
            }
        });
    }
}
