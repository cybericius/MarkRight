use std::collections::HashMap;

use comrak::nodes::{Ast, NodeValue};
use comrak::{Arena, Options, parse_document};
use serde::Serialize;

use crate::sanitize::{UrlKind, safe_url, sanitize_html};

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
    pub highlighted_html_light: Option<String>,
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
    /// Alert fold state: "open" or "closed" (Obsidian `[!type]+` / `[!type]-`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fold: Option<String>,
    /// Footnote reference: 1-based index of this reference to the same footnote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_num: Option<u32>,
    /// Footnote definition: total number of references to it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_count: Option<u32>,
}

impl MdNode {
    pub(crate) fn new(node_type: &str) -> Self {
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
            highlighted_html_light: None,
            list_type: None,
            start: None,
            tight: None,
            checked: None,
            header: None,
            alignments: None,
            fold: None,
            ref_num: None,
            ref_count: None,
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
            NodeValue::ShortCode(sc) => text.push_str(&sc.emoji),
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
    serialize_ast_themed(input, "ocean")
}

/// Parse Markdown and return a serializable AST with a specific code theme.
pub fn serialize_ast_themed(input: &str, code_theme: &str) -> MdNode {
    let arena = Arena::new();
    let mut options = Options::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;
    options.extension.footnotes = true;
    options.extension.math_dollars = true;
    options.extension.math_code = true;
    options.extension.shortcodes = true;
    options.render.unsafe_ = true;

    let root = parse_document(&arena, input, &options);
    let mut id_counts: HashMap<String, usize> = HashMap::new();

    let mut doc = convert_node(root, &mut id_counts, code_theme);
    gather_footnotes(&mut doc);
    doc
}

fn collect_footnote_numbers(node: &MdNode, numbers: &mut HashMap<String, usize>) {
    if node.node_type == "FootnoteReference"
        && let (Some(id), Some(n)) = (&node.id, node.start)
    {
        numbers.entry(id.clone()).or_insert(n);
    }
    for c in &node.children {
        collect_footnote_numbers(c, numbers);
    }
}

fn take_footnote_definitions(node: &mut MdNode, out: &mut Vec<MdNode>) {
    let children = std::mem::take(&mut node.children);
    for mut c in children {
        if c.node_type == "FootnoteDefinition" {
            out.push(c);
        } else {
            take_footnote_definitions(&mut c, out);
            node.children.push(c);
        }
    }
}

/// Number the footnote definitions like their references and move them into a
/// single `FootnoteSection` that is the last child of the document, ordered by
/// number. Definitions without a reference are dropped.
fn gather_footnotes(doc: &mut MdNode) {
    let mut numbers = HashMap::new();
    collect_footnote_numbers(doc, &mut numbers);
    let mut defs = Vec::new();
    take_footnote_definitions(doc, &mut defs);
    let mut defs: Vec<MdNode> = defs
        .into_iter()
        .filter_map(|mut d| {
            d.start = Some(*numbers.get(d.id.as_deref()?)?);
            Some(d)
        })
        .collect();
    if defs.is_empty() {
        return;
    }
    defs.sort_by_key(|d| d.start);
    let mut section = MdNode::new("FootnoteSection");
    section.children = defs;
    doc.children.push(section);
}

#[allow(clippy::too_many_lines)]
fn convert_node<'a>(
    node: &'a comrak::arena_tree::Node<'a, std::cell::RefCell<Ast>>,
    id_counts: &mut HashMap<String, usize>,
    code_theme: &str,
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
                .map(|c| convert_node(c, id_counts, code_theme))
                .collect();
            return n;
        }
        NodeValue::Paragraph => MdNode::new("Paragraph"),
        NodeValue::Text(t) => {
            let mut n = MdNode::new("Text");
            n.literal = Some(t.clone());
            n
        }
        NodeValue::ShortCode(sc) => {
            let mut n = MdNode::new("Text");
            n.literal = Some(sc.emoji.clone());
            n
        }
        NodeValue::Math(m) => {
            let mut n = MdNode::new("Math");
            n.literal = Some(m.literal.clone());
            n.info = Some(if m.display_math { "display" } else { "inline" }.to_string());
            n
        }
        NodeValue::FootnoteReference(r) => {
            let mut n = MdNode::new("FootnoteReference");
            n.id = Some(r.name.clone());
            n.start = Some(r.ix as usize);
            n.ref_num = Some(r.ref_num);
            n
        }
        NodeValue::FootnoteDefinition(d) => {
            let mut n = MdNode::new("FootnoteDefinition");
            n.id = Some(d.name.clone());
            n.ref_count = Some(d.total_references);
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
            if cb.info.split_whitespace().next() == Some("math") {
                n.node_type = "Math".to_string();
                n.info = Some("display".to_string());
                return n;
            }
            let lang = cb.info.split_whitespace().next().unwrap_or("").to_string();
            if !lang.is_empty() {
                n.info = Some(lang.clone());
                if let Ok((dark, light)) = markright_syntax::highlight_pair(&cb.literal, &lang, code_theme) {
                    n.highlighted_html = Some(dark);
                    n.highlighted_html_light = Some(light);
                }
            }
            n
        }
        NodeValue::HtmlBlock(h) => {
            let mut n = MdNode::new("HtmlBlock");
            n.literal = Some(sanitize_html(&h.literal));
            n
        }
        NodeValue::HtmlInline(h) => {
            let mut n = MdNode::new("HtmlInline");
            n.literal = Some(sanitize_html(h));
            n
        }
        NodeValue::Emph => MdNode::new("Emph"),
        NodeValue::Strong => MdNode::new("Strong"),
        NodeValue::Strikethrough => MdNode::new("Strikethrough"),
        NodeValue::Link(link) => {
            let mut n = MdNode::new("Link");
            n.url = safe_url(&link.url, UrlKind::Link);
            if !link.title.is_empty() {
                n.title = Some(link.title.clone());
            }
            n
        }
        NodeValue::Image(img) => {
            let mut n = MdNode::new("Image");
            n.url = safe_url(&img.url, UrlKind::Image);
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
        .map(|c| convert_node(c, id_counts, code_theme))
        .collect();

    if md_node.node_type == "BlockQuote" {
        crate::alert::convert_blockquote(&mut md_node);
    }

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

    fn walk<'n>(node: &'n MdNode, out: &mut Vec<&'n MdNode>) {
        out.push(node);
        for c in &node.children {
            walk(c, out);
        }
    }

    #[test]
    fn test_raw_html_is_sanitized() {
        let ast = serialize_ast("<img src=x onerror=alert(1)>\n\ntext <span onclick=\"x()\">hi</span>\n");
        let mut nodes = Vec::new();
        walk(&ast, &mut nodes);
        let html: Vec<_> = nodes
            .iter()
            .filter(|n| n.node_type.starts_with("Html"))
            .filter_map(|n| n.literal.as_deref())
            .collect();
        assert!(html.len() >= 2, "{html:?}");
        assert!(html.iter().all(|h| !h.contains("onerror") && !h.contains("onclick")), "{html:?}");
    }

    #[test]
    fn test_dangerous_link_and_image_urls_dropped() {
        let ast = serialize_ast("[a](javascript:alert(1)) ![b](vbscript:x) [c](other.md) ![d](img/d.png)\n");
        let mut nodes = Vec::new();
        walk(&ast, &mut nodes);
        let urls: Vec<_> = nodes
            .iter()
            .filter(|n| n.node_type == "Link" || n.node_type == "Image")
            .map(|n| n.url.as_deref())
            .collect();
        assert_eq!(urls, vec![None, None, Some("other.md"), Some("img/d.png")]);
    }

    fn find<'n>(ast: &'n MdNode, ty: &str) -> Vec<&'n MdNode> {
        let mut nodes = Vec::new();
        walk(ast, &mut nodes);
        nodes.into_iter().filter(|n| n.node_type == ty).collect()
    }

    #[test]
    fn test_footnote_numbering_and_section() {
        let ast = serialize_ast("A[^b] and B[^a] and again[^b].\n\n[^a]: Alpha\n[^b]: Beta\n");
        let refs = find(&ast, "FootnoteReference");
        let got: Vec<_> = refs.iter().map(|r| (r.id.clone().unwrap(), r.start, r.ref_num)).collect();
        assert_eq!(
            got,
            vec![
                ("b".to_string(), Some(1), Some(1)),
                ("a".to_string(), Some(2), Some(1)),
                ("b".to_string(), Some(1), Some(2)),
            ]
        );
        let last = ast.children.last().unwrap();
        assert_eq!(last.node_type, "FootnoteSection");
        assert_eq!(find(&ast, "FootnoteSection").len(), 1);
        let defs: Vec<_> = last.children.iter().map(|d| (d.id.clone().unwrap(), d.start, d.ref_count)).collect();
        assert_eq!(defs, vec![("b".to_string(), Some(1), Some(2)), ("a".to_string(), Some(2), Some(1))]);
    }

    #[test]
    fn test_footnote_definition_multiple_paragraphs() {
        let ast = serialize_ast("x[^1]\n\n[^1]: First\n\n    Second\n");
        let defs = find(&ast, "FootnoteDefinition");
        assert_eq!(defs.len(), 1);
        let paras = defs[0].children.iter().filter(|c| c.node_type == "Paragraph").count();
        assert_eq!(paras, 2);
    }

    #[test]
    fn test_footnote_unreferenced_definition_dropped() {
        let ast = serialize_ast("text\n\n[^zz]: never used\n");
        assert!(find(&ast, "FootnoteDefinition").is_empty());
        assert!(find(&ast, "FootnoteSection").is_empty());
    }

    #[test]
    fn test_footnote_label_case_insensitive() {
        // CommonMark-style labels match case-insensitively: [^Note] -> [^note]:
        let ast = serialize_ast("x[^Note]\n\n[^note]: body\n");
        let defs = find(&ast, "FootnoteDefinition");
        assert_eq!(defs.len(), 1, "definition must not be dropped");
        let r = &find(&ast, "FootnoteReference")[0];
        assert_eq!(r.id, defs[0].id, "reference and definition ids must match for the DOM links");
    }

    #[test]
    fn test_no_footnotes_no_section() {
        let ast = serialize_ast("plain\n");
        assert!(find(&ast, "FootnoteSection").is_empty());
    }

    #[test]
    fn test_footnote_json_shape() {
        let ast = serialize_ast("x[^1]\n\n[^1]: d\n");
        let v = serde_json::to_value(&ast).unwrap();
        let r = &v["children"][0]["children"][1];
        assert_eq!(r["type"], "FootnoteReference");
        assert_eq!(r["ref_num"], 1);
        assert_eq!(r["start"], 1);
        assert!(r.get("ref_count").is_none());
    }

    #[test]
    fn test_math_inline_display_and_fence() {
        let ast = serialize_ast("a $x^2$ b $`y`$ c\n\n$$z$$\n\n```math\nE=mc^2\n```\n");
        let m: Vec<_> = find(&ast, "Math")
            .iter()
            .map(|n| (n.literal.clone().unwrap(), n.info.clone().unwrap()))
            .collect();
        assert_eq!(m[0], ("x^2".to_string(), "inline".to_string()));
        assert_eq!(m[1], ("y".to_string(), "inline".to_string()));
        assert_eq!(m[2], ("z".to_string(), "display".to_string()));
        assert_eq!(m[3].1, "display");
        assert!(m[3].0.contains("E=mc^2"));
        assert_eq!(m.len(), 4);
        assert!(find(&ast, "CodeBlock").is_empty());
        assert!(find(&ast, "Math").iter().all(|n| n.highlighted_html.is_none()));
    }

    #[test]
    fn test_currency_stays_text() {
        let ast = serialize_ast("It costs $5 and $10 today.\n");
        assert!(find(&ast, "Math").is_empty());
        let text: String = find(&ast, "Text").iter().filter_map(|n| n.literal.as_deref()).collect();
        assert_eq!(text, "It costs $5 and $10 today.");
    }

    #[test]
    fn test_shortcodes() {
        let ast = serialize_ast("hi :smile: and :notanemoji:\n");
        let text: String = find(&ast, "Text").iter().filter_map(|n| n.literal.as_deref()).collect();
        assert_eq!(text, "hi \u{1F604} and :notanemoji:");
        assert!(find(&ast, "Unknown").is_empty());
    }

    #[test]
    fn test_shortcode_in_heading_toc() {
        let ast = serialize_ast("# Party :tada:\n");
        let toc = crate::toc::extract_toc_from_ast(&ast);
        assert_eq!(toc[0].text, "Party \u{1F389}");
    }
}
