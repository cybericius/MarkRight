use comrak::{Arena, Options, parse_document};
use serde::Serialize;

/// Parsed Markdown document represented as a JSON-serializable AST.
#[derive(Debug, Serialize)]
pub struct ParsedDocument {
    pub html: String,
}

/// Parse a Markdown string and return rendered HTML.
///
/// This is the initial simple implementation. It will be replaced with
/// full AST-as-JSON serialization in Phase 1a.
///
/// # Panics
///
/// Panics if comrak produces invalid UTF-8 output (should never happen).
pub fn parse_markdown(input: &str) -> ParsedDocument {
    let arena = Arena::new();
    let mut options = Options::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;
    options.render.unsafe_ = true;

    let root = parse_document(&arena, input, &options);

    let mut html = Vec::new();
    comrak::format_html(root, &options, &mut html).unwrap();

    ParsedDocument {
        html: String::from_utf8(html).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_markdown() {
        let result = parse_markdown("# Hello\n\nWorld");
        assert!(result.html.contains("<h1>"));
        assert!(result.html.contains("Hello"));
        assert!(result.html.contains("World"));
    }

    #[test]
    fn test_parse_gfm_table() {
        let input = "| A | B |\n|---|---|\n| 1 | 2 |";
        let result = parse_markdown(input);
        assert!(result.html.contains("<table>"));
    }

    #[test]
    fn test_parse_tasklist() {
        let input = "- [x] Done\n- [ ] Todo";
        let result = parse_markdown(input);
        assert!(result.html.contains("checked"));
    }
}
