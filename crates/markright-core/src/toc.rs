use serde::Serialize;

use crate::ast::MdNode;

/// A heading entry for the Table of Contents.
#[derive(Debug, Clone, Serialize)]
pub struct TocEntry {
    pub level: u8,
    pub text: String,
    pub id: String,
}

/// Extract headings from a serialized `MdNode` AST tree.
pub fn extract_toc_from_ast(root: &MdNode) -> Vec<TocEntry> {
    let mut entries = Vec::new();
    collect_headings(root, &mut entries);
    entries
}

fn collect_headings(node: &MdNode, entries: &mut Vec<TocEntry>) {
    if node.node_type == "Heading"
        && let (Some(level), Some(id)) = (node.level, node.id.as_deref())
    {
        let text = collect_node_text(node);
        entries.push(TocEntry {
            level,
            text,
            id: id.to_string(),
        });
    }
    for child in &node.children {
        collect_headings(child, entries);
    }
}

/// Collect plain text from an `MdNode` subtree.
fn collect_node_text(node: &MdNode) -> String {
    let mut text = String::new();
    if let Some(ref lit) = node.literal {
        text.push_str(lit);
    }
    for child in &node.children {
        text.push_str(&collect_node_text(child));
    }
    text
}

/// Extract headings from Markdown source for TOC generation.
///
/// Simple regex-based implementation. Prefer `extract_toc_from_ast` for
/// accurate results based on the parsed AST.
#[deprecated(note = "use extract_toc_from_ast instead")]
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
                && let Some(text) = remaining.strip_prefix(' ')
            {
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
    use crate::ast::serialize_ast;

    #[test]
    fn test_extract_toc_from_ast() {
        let ast = serialize_ast("# Title\n\nSome text\n\n## Section 1\n\n### Sub 1.1\n\n## Section 2");
        let toc = extract_toc_from_ast(&ast);
        assert_eq!(toc.len(), 4);
        assert_eq!(toc[0].level, 1);
        assert_eq!(toc[0].text, "Title");
        assert_eq!(toc[0].id, "title");
        assert_eq!(toc[1].level, 2);
        assert_eq!(toc[1].text, "Section 1");
        assert_eq!(toc[2].level, 3);
        assert_eq!(toc[2].text, "Sub 1.1");
        assert_eq!(toc[3].level, 2);
        assert_eq!(toc[3].text, "Section 2");
    }

    #[test]
    fn test_duplicate_heading_ids_in_toc() {
        let ast = serialize_ast("# Foo\n\n# Foo\n\n# Foo");
        let toc = extract_toc_from_ast(&ast);
        assert_eq!(toc.len(), 3);
        assert_eq!(toc[0].id, "foo");
        assert_eq!(toc[1].id, "foo-1");
        assert_eq!(toc[2].id, "foo-2");
    }

    #[allow(deprecated)]
    #[test]
    fn test_extract_toc_legacy() {
        let md = "# Title\n\nSome text\n\n## Section 1\n\n### Sub 1.1\n\n## Section 2";
        let toc = extract_toc(md);
        assert_eq!(toc.len(), 4);
        assert_eq!(toc[0].level, 1);
        assert_eq!(toc[0].text, "Title");
        assert_eq!(toc[1].level, 2);
        assert_eq!(toc[2].level, 3);
    }
}
