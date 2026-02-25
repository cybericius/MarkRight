use serde::Serialize;
use std::collections::HashMap;

/// Parsed frontmatter from a Markdown document.
#[derive(Debug, Default, Serialize)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub fields: HashMap<String, String>,
}

/// Strip YAML frontmatter from Markdown content.
/// Returns the frontmatter (if present) and the remaining content.
pub fn strip_frontmatter(content: &str) -> (Option<Frontmatter>, &str) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return (None, content);
    }

    // Find the closing ---
    if let Some(end) = trimmed[3..].find("\n---") {
        let yaml_block = &trimmed[3..end + 3].trim();
        let rest = &trimmed[end + 3 + 4..]; // skip past "\n---"

        let mut fm = Frontmatter::default();
        for line in yaml_block.lines() {
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim().to_string();
                let value = value.trim().to_string();
                if key == "title" {
                    fm.title = Some(value.trim_matches('"').trim_matches('\'').to_string());
                }
                fm.fields.insert(key, value);
            }
        }

        (Some(fm), rest)
    } else {
        (None, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_frontmatter() {
        let content = "---\ntitle: Hello World\nauthor: Test\n---\n# Hello";
        let (fm, rest) = strip_frontmatter(content);
        assert!(fm.is_some());
        let fm = fm.unwrap();
        assert_eq!(fm.title, Some("Hello World".to_string()));
        assert!(rest.contains("# Hello"));
    }

    #[test]
    fn test_no_frontmatter() {
        let content = "# Just a heading\n\nSome text";
        let (fm, rest) = strip_frontmatter(content);
        assert!(fm.is_none());
        assert_eq!(rest, content);
    }
}
