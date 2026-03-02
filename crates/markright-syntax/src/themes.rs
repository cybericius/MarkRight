use syntect::highlighting::{
    Color, ScopeSelectors, StyleModifier, Theme, ThemeItem, ThemeSettings,
};

fn color(hex: u32) -> Color {
    Color {
        r: ((hex >> 16) & 0xFF) as u8,
        g: ((hex >> 8) & 0xFF) as u8,
        b: (hex & 0xFF) as u8,
        a: 0xFF,
    }
}

fn scope(s: &str) -> ScopeSelectors {
    s.parse().unwrap()
}

fn item(scope_str: &str, fg: Color) -> ThemeItem {
    ThemeItem {
        scope: scope(scope_str),
        style: StyleModifier {
            foreground: Some(fg),
            background: None,
            font_style: None,
        },
    }
}

fn item_italic(scope_str: &str, fg: Color) -> ThemeItem {
    ThemeItem {
        scope: scope(scope_str),
        style: StyleModifier {
            foreground: Some(fg),
            background: None,
            font_style: Some(syntect::highlighting::FontStyle::ITALIC),
        },
    }
}

/// Atelier Sulphurpool base16 palette.
struct Sulphurpool;

#[allow(clippy::unreadable_literal)]
impl Sulphurpool {
    const BASE00: u32 = 0x202746; // darkest bg
    const BASE01: u32 = 0x293256;
    const BASE02: u32 = 0x5e6687;
    const BASE03: u32 = 0x6b7394; // comments
    const BASE04: u32 = 0x898ea4;
    const BASE05: u32 = 0x979db4; // dark fg
    const BASE06: u32 = 0xdfe2f1;
    const BASE07: u32 = 0xf5f7ff; // lightest bg
    const BASE08: u32 = 0xc94922; // red: variables, tags
    const BASE09: u32 = 0xc76b29; // orange: integers, booleans, constants
    const BASE0A: u32 = 0xc08b30; // yellow: classes
    const BASE0B: u32 = 0xac9739; // green: strings
    const BASE0C: u32 = 0x22a2c9; // cyan: support, regex
    const BASE0D: u32 = 0x3d8fd1; // blue: functions
    const BASE0E: u32 = 0x6679cc; // violet: keywords
    const BASE0F: u32 = 0x9c637a; // magenta: deprecated, embedded
}

fn sulphurpool_scopes() -> Vec<ThemeItem> {
    vec![
        item_italic("comment", color(Sulphurpool::BASE03)),
        item("string", color(Sulphurpool::BASE0B)),
        item("constant.numeric", color(Sulphurpool::BASE09)),
        item("constant.language", color(Sulphurpool::BASE09)),
        item("constant.character, constant.other", color(Sulphurpool::BASE09)),
        item("variable", color(Sulphurpool::BASE08)),
        item("keyword", color(Sulphurpool::BASE0E)),
        item("keyword.operator", color(Sulphurpool::BASE0E)),
        item("storage", color(Sulphurpool::BASE0E)),
        item("storage.type", color(Sulphurpool::BASE0C)),
        item("entity.name.class, entity.name.type", color(Sulphurpool::BASE0A)),
        item("entity.name.function", color(Sulphurpool::BASE0D)),
        item("entity.name.tag", color(Sulphurpool::BASE08)),
        item("entity.other.attribute-name", color(Sulphurpool::BASE09)),
        item("support.function", color(Sulphurpool::BASE0C)),
        item("support.constant", color(Sulphurpool::BASE09)),
        item("support.type, support.class", color(Sulphurpool::BASE0A)),
        item("punctuation.definition.string", color(Sulphurpool::BASE0B)),
        item("punctuation.section", color(Sulphurpool::BASE02)),
        item("meta.function-call", color(Sulphurpool::BASE0D)),
        item("invalid.deprecated", color(Sulphurpool::BASE0F)),
    ]
}

/// Sulphurpool dark theme (dark background, light foreground).
pub fn sulphurpool_dark() -> Theme {
    Theme {
        name: Some("Sulphurpool Dark".to_owned()),
        author: Some("Bram de Haan (adapted)".to_owned()),
        settings: ThemeSettings {
            foreground: Some(color(Sulphurpool::BASE05)),
            background: Some(color(Sulphurpool::BASE00)),
            caret: Some(color(Sulphurpool::BASE05)),
            selection: Some(color(Sulphurpool::BASE02)),
            line_highlight: Some(color(Sulphurpool::BASE01)),
            gutter: Some(color(Sulphurpool::BASE00)),
            gutter_foreground: Some(color(Sulphurpool::BASE03)),
            ..ThemeSettings::default()
        },
        scopes: sulphurpool_scopes(),
    }
}

/// Sulphurpool light theme (light background, dark foreground).
pub fn sulphurpool_light() -> Theme {
    Theme {
        name: Some("Sulphurpool Light".to_owned()),
        author: Some("Bram de Haan (adapted)".to_owned()),
        settings: ThemeSettings {
            foreground: Some(color(Sulphurpool::BASE02)),
            background: Some(color(Sulphurpool::BASE07)),
            caret: Some(color(Sulphurpool::BASE02)),
            selection: Some(color(Sulphurpool::BASE06)),
            line_highlight: Some(color(Sulphurpool::BASE06)),
            gutter: Some(color(Sulphurpool::BASE07)),
            gutter_foreground: Some(color(Sulphurpool::BASE04)),
            ..ThemeSettings::default()
        },
        scopes: sulphurpool_scopes(),
    }
}
