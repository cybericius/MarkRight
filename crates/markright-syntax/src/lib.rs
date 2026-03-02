mod themes;

use std::collections::HashMap;
use std::sync::LazyLock;

use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);

/// Custom themes not bundled with syntect.
static CUSTOM_THEMES: LazyLock<HashMap<&str, Theme>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("sulphurpool-dark", themes::sulphurpool_dark());
    m.insert("sulphurpool-light", themes::sulphurpool_light());
    m
});

/// Known code theme names. Each maps to a (dark, light) theme pair.
pub const CODE_THEMES: &[(&str, &str)] = &[
    ("ocean", "Base16 Ocean"),
    ("sulphurpool", "Sulphurpool"),
];

/// Returns the (dark, light) syntect theme key pair for a code theme name.
fn theme_pair(code_theme: &str) -> (&'static str, &'static str) {
    match code_theme {
        "sulphurpool" => ("sulphurpool-dark", "sulphurpool-light"),
        // "ocean" or anything else falls back to the originals
        _ => ("base16-ocean.dark", "InspiredGitHub"),
    }
}

fn resolve_theme(name: &str) -> &Theme {
    CUSTOM_THEMES
        .get(name)
        .unwrap_or_else(|| &THEME_SET.themes[name])
}

/// Error type for syntax highlighting failures.
#[derive(Debug)]
pub enum HighlightError {
    /// The requested language is not supported.
    UnknownLanguage(String),
    /// Syntect returned an error during highlighting.
    Syntect(syntect::Error),
}

impl std::fmt::Display for HighlightError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownLanguage(lang) => write!(f, "unknown language: {lang}"),
            Self::Syntect(e) => write!(f, "syntect error: {e}"),
        }
    }
}

impl std::error::Error for HighlightError {}

impl From<syntect::Error> for HighlightError {
    fn from(e: syntect::Error) -> Self {
        Self::Syntect(e)
    }
}

/// Highlight for the dark variant of a code theme.
///
/// # Errors
///
/// Returns `HighlightError::UnknownLanguage` if the language is not recognized.
pub fn highlight(code: &str, language: &str) -> Result<String, HighlightError> {
    highlight_themed(code, language, "base16-ocean.dark")
}

/// Highlight for the light variant of a code theme.
///
/// # Errors
///
/// Returns `HighlightError::UnknownLanguage` if the language is not recognized.
pub fn highlight_light(code: &str, language: &str) -> Result<String, HighlightError> {
    highlight_themed(code, language, "InspiredGitHub")
}

/// Highlight with a specific code theme name (e.g. "ocean", "sulphurpool").
/// Produces both dark and light HTML variants.
///
/// # Errors
///
/// Returns `HighlightError::UnknownLanguage` if the language is not recognized.
pub fn highlight_pair(
    code: &str,
    language: &str,
    code_theme: &str,
) -> Result<(String, String), HighlightError> {
    let (dark_name, light_name) = theme_pair(code_theme);
    let dark = highlight_themed(code, language, dark_name)?;
    let light = highlight_themed(code, language, light_name)?;
    Ok((dark, light))
}

fn highlight_themed(
    code: &str,
    language: &str,
    theme_name: &str,
) -> Result<String, HighlightError> {
    let syntax = SYNTAX_SET
        .find_syntax_by_token(language)
        .ok_or_else(|| HighlightError::UnknownLanguage(language.to_string()))?;

    let theme = resolve_theme(theme_name);
    let html = highlighted_html_for_string(code, &SYNTAX_SET, syntax, theme)?;
    Ok(html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_rust() {
        let result = highlight("fn main() {}", "rust");
        assert!(result.is_ok());
        let html = result.unwrap();
        assert!(html.contains("style="));
        assert!(html.contains("main"));
    }

    #[test]
    fn test_highlight_unknown_language() {
        let result = highlight("code", "nonexistent_lang_xyz");
        assert!(result.is_err());
    }

    #[test]
    fn test_highlight_javascript() {
        let result = highlight("const x = 42;", "js");
        assert!(result.is_ok());
    }

    #[test]
    fn test_highlight_pair_ocean() {
        let (dark, light) = highlight_pair("fn main() {}", "rust", "ocean").unwrap();
        assert!(dark.contains("style="));
        assert!(light.contains("style="));
        assert_ne!(dark, light);
    }

    #[test]
    fn test_highlight_pair_sulphurpool() {
        let (dark, light) = highlight_pair("fn main() {}", "rust", "sulphurpool").unwrap();
        assert!(dark.contains("style="));
        assert!(light.contains("style="));
        assert_ne!(dark, light);
    }
}
