use std::sync::LazyLock;

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);

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

/// Highlight source code and return HTML with inline styles.
///
/// # Errors
///
/// Returns `HighlightError::UnknownLanguage` if the language is not recognized.
pub fn highlight(code: &str, language: &str) -> Result<String, HighlightError> {
    let syntax = SYNTAX_SET
        .find_syntax_by_token(language)
        .ok_or_else(|| HighlightError::UnknownLanguage(language.to_string()))?;

    let theme = &THEME_SET.themes["base16-ocean.dark"];
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
}
