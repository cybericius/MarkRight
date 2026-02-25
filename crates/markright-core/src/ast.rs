use std::collections::HashMap;

use comrak::nodes::{Ast, NodeValue};
use comrak::{Arena, Options, parse_document};
use serde::Serialize;

/// A serializable Markdown AST node.
///
/// Uses a flat struct with a `type` discriminator and optional fields rather than
/// enum tagging, which simplifies frontend consumption.
#[derive(Debug, Clone, Serialize)]
pub struct MdNode {
    #[serde(rename = "type")]
    pub node_type: String,
    pub children: Vec<MdNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub literal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlighted_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tight: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignments: Option<Vec<String>>,
}

impl MdNode {
    fn new(node_type: &str) -> Self {
        Self {
            node_type: node_type.to_string(),
            children: Vec::new(),
            level: None,
            id: None,
            literal: None,
            url: None,
            title: None,
            info: None,
            highlighted_html: None,
            list_type: None,
            start: None,
            tight: None,
            checked: None,
            header: None,
            alignments: None,
        }
    }
}

/// Extract plain text content from a comrak AST node (recursive).
fn collect_text<'a>(node: &'a comrak::arena_tree::Node<'a, std::cell::RefCell<Ast>>) -> String {
    let mut text = String::new();
    for child in node.children() {
        let data = child.data.borrow();
        match &data.value {
            NodeValue::Text(t) => text.push_str(t),
            NodeValue::Code(c) => text.push_str(&c.literal),
            _ => {}
        }
        drop(data);
        text.push_str(&collect_text(child));
    }
    text
}

/// Generate a URL-safe slug from heading text.
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c == ' ' || c == '-' {
                '-'
            } else {
                '\0'
            }
        })
        .filter(|&c| c != '\0')
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Parse Markdown and return a serializable AST.
pub fn serialize_ast(input: &str) -> MdNode {
    let arena = Arena::new();
    let mut options = Options::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;
    options.render.unsafe_ = true;

    let root = parse_document(&arena, input, &options);
    let mut id_counts: HashMap<String, usize> = HashMap::new();

    convert_node(root, &mut id_counts)
}

#[allow(clippy::too_many_lines)]
fn convert_node<'a>(
    node: &'a comrak::arena_tree::Node<'a, std::cell::RefCell<Ast>>,
    id_counts: &mut HashMap<String, usize>,
) -> MdNode {
    let data = node.data.borrow();
    let mut md_node = match &data.value {
        NodeValue::Document => MdNode::new("Document"),
        NodeValue::Heading(h) => {
            let level = h.level;
            drop(data);
            let text = collect_text(node);
            let base_slug = slugify(&text);
            let count = id_counts.entry(base_slug.clone()).or_insert(0);
            let id = if *count == 0 {
                base_slug
            } else {
                format!("{base_slug}-{count}")
            };
            let entry = id_counts.get_mut(&slugify(&text)).unwrap();
            *entry += 1;

            let mut n = MdNode::new("Heading");
            n.level = Some(level);
            n.id = Some(id);
            n.children = node
                .children()
                .map(|c| convert_node(c, id_counts))
                .collect();
            return n;
        }
        NodeValue::Paragraph => MdNode::new("Paragraph"),
        NodeValue::Text(t) => {
            let mut n = MdNode::new("Text");
            n.literal = Some(t.clone());
            n
        }
        NodeValue::SoftBreak => MdNode::new("SoftBreak"),
        NodeValue::LineBreak => MdNode::new("LineBreak"),
        NodeValue::ThematicBreak => MdNode::new("ThematicBreak"),
        NodeValue::BlockQuote => MdNode::new("BlockQuote"),
        NodeValue::Code(c) => {
            let mut n = MdNode::new("Code");
            n.literal = Some(c.literal.clone());
            n
        }
        NodeValue::CodeBlock(cb) => {
            let mut n = MdNode::new("CodeBlock");
            n.literal = Some(cb.literal.clone());
            let lang = cb.info.split_whitespace().next().unwrap_or("").to_string();
            if !lang.is_empty() {
                n.info = Some(lang.clone());
                if let Ok(html) = markright_syntax::highlight(&cb.literal, &lang) {
                    n.highlighted_html = Some(html);
                }
            }
            n
        }
        NodeValue::HtmlBlock(h) => {
            let mut n = MdNode::new("HtmlBlock");
            n.literal = Some(h.literal.clone());
            n
        }
        NodeValue::HtmlInline(h) => {
            let mut n = MdNode::new("HtmlInline");
            n.literal = Some(h.clone());
            n
        }
        NodeValue::Emph => MdNode::new("Emph"),
        NodeValue::Strong => MdNode::new("Strong"),
        NodeValue::Strikethrough => MdNode::new("Strikethrough"),
        NodeValue::Link(link) => {
            let mut n = MdNode::new("Link");
            n.url = Some(link.url.clone());
            if !link.title.is_empty() {
                n.title = Some(link.title.clone());
            }
            n
        }
        NodeValue::Image(img) => {
            let mut n = MdNode::new("Image");
            n.url = Some(img.url.clone());
            if !img.title.is_empty() {
                n.title = Some(img.title.clone());
            }
            n
        }
        NodeValue::List(list) => {
            let mut n = MdNode::new("List");
            n.list_type = Some(
                match list.list_type {
                    comrak::nodes::ListType::Bullet => "bullet",
                    comrak::nodes::ListType::Ordered => "ordered",
                }
                .to_string(),
            );
            n.start = Some(list.start);
            n.tight = Some(list.tight);
            n
        }
        NodeValue::Item(_) => MdNode::new("Item"),
        NodeValue::TaskItem(checked) => {
            let mut n = MdNode::new("TaskItem");
            n.checked = Some(checked.is_some());
            n
        }
        NodeValue::Table(t) => {
            let mut n = MdNode::new("Table");
            n.alignments = Some(
                t.alignments
                    .iter()
                    .map(|a| match a {
                        comrak::nodes::TableAlignment::None => "none",
                        comrak::nodes::TableAlignment::Left => "left",
                        comrak::nodes::TableAlignment::Center => "center",
                        comrak::nodes::TableAlignment::Right => "right",
                    })
                    .map(String::from)
                    .collect(),
            );
            n
        }
        NodeValue::TableRow(header) => {
            let mut n = MdNode::new("TableRow");
            n.header = Some(*header);
            n
        }
        NodeValue::TableCell => MdNode::new("TableCell"),
        // Catch-all for other node types
        _ => MdNode::new("Unknown"),
    };
    drop(data);

    md_node.children = node
        .children()
        .map(|c| convert_node(c, id_counts))
        .collect();

    md_node
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_ast() {
        let ast = serialize_ast("# Hello\n\nWorld");
        assert_eq!(ast.node_type, "Document");
        assert_eq!(ast.children.len(), 2);

        let heading = &ast.children[0];
        assert_eq!(heading.node_type, "Heading");
        assert_eq!(heading.level, Some(1));
        assert_eq!(heading.id.as_deref(), Some("hello"));
    }

    #[test]
    fn test_duplicate_heading_ids() {
        let ast = serialize_ast("# Foo\n\n# Foo\n\n# Foo");
        let ids: Vec<&str> = ast
            .children
            .iter()
            .filter_map(|n| n.id.as_deref())
            .collect();
        assert_eq!(ids, vec!["foo", "foo-1", "foo-2"]);
    }

    #[test]
    fn test_code_block() {
        let ast = serialize_ast("```rust\nfn main() {}\n```");
        let code = &ast.children[0];
        assert_eq!(code.node_type, "CodeBlock");
        assert_eq!(code.info.as_deref(), Some("rust"));
        assert!(code.literal.is_some());
    }

    #[test]
    fn test_table() {
        let ast = serialize_ast("| A | B |\n|---|---|\n| 1 | 2 |");
        let table = &ast.children[0];
        assert_eq!(table.node_type, "Table");
        assert!(table.alignments.is_some());
    }

    #[test]
    fn test_tasklist() {
        let ast = serialize_ast("- [x] Done\n- [ ] Todo");
        let list = &ast.children[0];
        assert_eq!(list.node_type, "List");
        // Items contain TaskItem children
        let item0 = &list.children[0];
        assert_eq!(item0.node_type, "TaskItem");
        assert_eq!(item0.checked, Some(true));
    }

    #[test]
    fn test_link() {
        let ast = serialize_ast("[click](https://example.com)");
        let para = &ast.children[0];
        let link = &para.children[0];
        assert_eq!(link.node_type, "Link");
        assert_eq!(link.url.as_deref(), Some("https://example.com"));
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("What's New?"), "whats-new");
        assert_eq!(slugify("  Multiple   Spaces  "), "multiple-spaces");
    }
}
