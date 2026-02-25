use serde::Serialize;

/// A heading entry for the Table of Contents.
#[derive(Debug, Serialize)]
pub struct TocEntry {
    pub level: u8,
    pub text: String,
    pub id: String,
}

/// Extract headings from Markdown source for TOC generation.
///
/// This is a simple regex-based implementation for bootstrap.
/// Will be replaced with AST-based extraction in Phase 1a.
pub fn extract_toc(markdown: &str) -> Vec<TocEntry> {
    let mut entries = Vec::new();

    for line in markdown.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix('#') {
            let mut level: u8 = 1;
            let mut remaining = rest;
            while let Some(r) = remaining.strip_prefix('#') {
                level += 1;
                remaining = r;
                if level > 6 {
                    break;
                }
            }
            if level <= 6
                && let Some(text) = remaining.strip_prefix(' ') {
                    let text = text.trim().to_string();
                    let id = text
                        .to_lowercase()
                        .replace(|c: char| !c.is_alphanumeric() && c != ' ' && c != '-', "")
                        .replace(' ', "-");
                    entries.push(TocEntry { level, text, id });
                }
        }
    }

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_toc() {
        let md = "# Title\n\nSome text\n\n## Section 1\n\n### Sub 1.1\n\n## Section 2";
        let toc = extract_toc(md);
        assert_eq!(toc.len(), 4);
        assert_eq!(toc[0].level, 1);
        assert_eq!(toc[0].text, "Title");
        assert_eq!(toc[1].level, 2);
        assert_eq!(toc[2].level, 3);
    }
}
