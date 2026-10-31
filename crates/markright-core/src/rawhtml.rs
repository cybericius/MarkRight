//! Raw HTML in documents: pairing tags that wrap Markdown, then sanitizing.
//!
//! comrak emits every HTML tag that is separated by Markdown as its own node:
//! `<kbd>Ctrl</kbd>` is `HtmlInline` `<kbd>`, `Text`, `HtmlInline` `</kbd>`, and a
//! `<details>` with a Markdown body is an `HtmlBlock`, the body blocks, and an
//! `HtmlBlock` `</details>`. Rendered one by one, the tags never wrap their
//! content. `finalize` pairs such open/close tags into an `HtmlElement` node
//! whose children are the Markdown in between, then sanitizes every HTML
//! literal exactly once. It runs once over the finished tree, so it is the
//! single place where document HTML is sanitized.

use crate::ast::MdNode;
use crate::sanitize::sanitize_html;

const VOID: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr",
];

#[derive(Debug, PartialEq)]
enum Tag {
    Open { name: String, end: usize },
    Close { name: String, end: usize },
}

/// Tokenize the tags of an HTML fragment (start tags that can hold content,
/// and end tags). Comments, void and self-closing tags are skipped.
fn tags(html: &str) -> Vec<Tag> {
    let bytes = html.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'<' {
            i += 1;
            continue;
        }
        if html[i..].starts_with("<!--") {
            i = html[i..].find("-->").map_or(bytes.len(), |off| i + off + 3);
            continue;
        }
        let close = bytes.get(i + 1) == Some(&b'/');
        let name_start = i + 1 + usize::from(close);
        let name_len = html[name_start..]
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
            .unwrap_or(html.len() - name_start);
        if name_len == 0 || !bytes[name_start].is_ascii_alphabetic() {
            i += 1;
            continue;
        }
        let name = html[name_start..name_start + name_len].to_ascii_lowercase();
        // Find the closing '>' outside quoted attribute values.
        let mut j = name_start + name_len;
        let mut open_quote: Option<u8> = None;
        while j < bytes.len() {
            match (open_quote, bytes[j]) {
                (None, b'"' | b'\'') => open_quote = Some(bytes[j]),
                (Some(q), c) if c == q => open_quote = None,
                (None, b'>') => break,
                _ => {}
            }
            j += 1;
        }
        if j >= bytes.len() {
            break;
        }
        let end = j + 1;
        if close {
            out.push(Tag::Close { name, end });
        } else if !(VOID.contains(&name.as_str()) || bytes[j - 1] == b'/') {
            out.push(Tag::Open { name, end });
        }
        i = end;
    }
    out
}

/// `literal` (trimmed) is exactly one start tag: returns its name.
fn sole_open_tag(literal: &str) -> Option<String> {
    let t = literal.trim();
    match tags(t).as_slice() {
        [Tag::Open { name, end }] if t.starts_with('<') && *end == t.len() => Some(name.clone()),
        _ => None,
    }
}

/// A block that starts with `<name ...>` and leaves exactly that element
/// open: returns (name, end of the start tag).
fn block_opener(literal: &str) -> Option<(String, usize)> {
    let t = literal.trim_start();
    let offset = literal.len() - t.len();
    let toks = tags(t);
    let Some(Tag::Open { name, end }) = toks.first() else { return None };
    if !t.starts_with('<') {
        return None;
    }
    let mut stack: Vec<&str> = Vec::new();
    for tok in &toks {
        match tok {
            Tag::Open { name, .. } => stack.push(name),
            Tag::Close { name, .. } => {
                if stack.last() == Some(&name.as_str()) {
                    stack.pop();
                }
            }
        }
    }
    (stack == [name.as_str()]).then(|| (name.clone(), offset + end))
}

/// A block that starts with `</name>`: returns the end of that tag.
fn block_closer(literal: &str, name: &str) -> Option<usize> {
    let t = literal.trim_start();
    let offset = literal.len() - t.len();
    match tags(t).first() {
        Some(Tag::Close { name: n, end }) if n == name && t.starts_with("</") => Some(offset + end),
        _ => None,
    }
}

fn html_node(kind: &str, literal: &str) -> MdNode {
    let mut n = MdNode::new(kind);
    n.literal = Some(literal.to_string());
    n
}

/// Pair open/close tags among siblings into `HtmlElement` nodes.
fn merge(children: Vec<MdNode>) -> Vec<MdNode> {
    let mut out = Vec::with_capacity(children.len());
    let mut rest = children.into_iter().collect::<std::collections::VecDeque<_>>();
    while let Some(node) = rest.pop_front() {
        let literal = node.literal.as_deref().unwrap_or("");
        let pairing = match node.node_type.as_str() {
            "HtmlInline" => sole_open_tag(literal).map(|n| (n.clone(), literal.trim().len())),
            "HtmlBlock" => block_opener(literal),
            _ => None,
        };
        let Some((name, open_end)) = pairing else {
            out.push(node);
            continue;
        };

        // Find the matching close among the following siblings of the same kind.
        let mut depth = 0usize;
        let mut close_at = None;
        for (k, sib) in rest.iter().enumerate() {
            if sib.node_type != node.node_type {
                continue;
            }
            let lit = sib.literal.as_deref().unwrap_or("");
            let opens = if node.node_type == "HtmlInline" {
                sole_open_tag(lit).is_some_and(|n| n == name)
            } else {
                block_opener(lit).is_some_and(|(n, _)| n == name)
            };
            let closes = if node.node_type == "HtmlInline" {
                lit.trim().eq_ignore_ascii_case(&format!("</{name}>"))
            } else {
                block_closer(lit, &name).is_some()
            };
            if opens {
                depth += 1;
            } else if closes {
                if depth == 0 {
                    close_at = Some(k);
                    break;
                }
                depth -= 1;
            }
        }
        let Some(k) = close_at else {
            out.push(node);
            continue;
        };

        let raw = literal.to_string();
        let inner: Vec<MdNode> = rest.drain(..k).collect();
        let closer = rest.pop_front().expect("found above");

        let mut el = MdNode::new("HtmlElement");
        el.info = Some(name.clone());
        // The start tag alone; sanitized (with its closing tag) in `finalize`.
        el.literal = Some(raw[..open_end].trim().to_string());
        let remainder = raw[open_end..].trim();
        if !remainder.is_empty() {
            el.children.push(html_node(&node.node_type, remainder));
        }
        el.children.extend(inner);
        el.children = merge(std::mem::take(&mut el.children));
        out.push(el);

        if node.node_type == "HtmlBlock" {
            let lit = closer.literal.as_deref().unwrap_or("");
            let after = block_closer(lit, &name).map_or("", |e| lit[e..].trim());
            if !after.is_empty() {
                rest.push_front(html_node("HtmlBlock", after));
            }
        }
    }
    out
}

/// Pair wrapping tags and sanitize all document HTML in the tree.
pub(crate) fn finalize(node: &mut MdNode) {
    node.children = merge(std::mem::take(&mut node.children));
    let mut unwrapped = Vec::with_capacity(node.children.len());
    for mut child in std::mem::take(&mut node.children) {
        match child.node_type.as_str() {
            "HtmlBlock" | "HtmlInline" => {
                child.literal = Some(sanitize_html(child.literal.as_deref().unwrap_or("")));
            }
            "HtmlElement" => {
                let name = child.info.clone().unwrap_or_default();
                let start = child.literal.as_deref().unwrap_or("");
                let shell = sanitize_html(&format!("{start}</{name}>"));
                if shell.trim().is_empty() {
                    // The sanitizer removed the element itself (e.g. <iframe>):
                    // keep the Markdown inside, drop the wrapper.
                    finalize(&mut child);
                    unwrapped.extend(child.children);
                    continue;
                }
                child.literal = Some(shell);
            }
            _ => {}
        }
        finalize(&mut child);
        unwrapped.push(child);
    }
    node.children = unwrapped;
}

#[cfg(test)]
mod tests {
    use crate::ast::{MdNode, serialize_ast};

    fn walk<'n>(n: &'n MdNode, out: &mut Vec<&'n MdNode>) {
        out.push(n);
        for c in &n.children {
            walk(c, out);
        }
    }

    fn all(md: &str) -> (MdNode, Vec<String>) {
        let ast = serialize_ast(md);
        let mut nodes = Vec::new();
        walk(&ast, &mut nodes);
        let html = nodes
            .iter()
            .filter(|n| n.node_type.starts_with("Html"))
            .map(|n| format!("{}:{}", n.node_type, n.literal.as_deref().unwrap_or("")))
            .collect();
        (ast, html)
    }

    fn elements(ast: &MdNode) -> Vec<String> {
        let mut nodes = Vec::new();
        walk(ast, &mut nodes);
        nodes.iter().filter(|n| n.node_type == "HtmlElement").map(|n| n.info.clone().unwrap()).collect()
    }

    #[test]
    fn inline_tags_wrap_their_markdown() {
        let (ast, html) = all("Press <kbd>Ctrl</kbd> and <span>**bold** text</span>.\n");
        assert_eq!(elements(&ast), vec!["kbd", "span"]);
        assert!(html.contains(&"HtmlElement:<kbd></kbd>".to_string()), "{html:?}");
        let kbd = ast.children[0].children.iter().find(|n| n.node_type == "HtmlElement").unwrap();
        assert_eq!(kbd.children[0].literal.as_deref(), Some("Ctrl"));
    }

    #[test]
    fn details_block_wraps_markdown_body() {
        let md = "<details open>\n<summary>More</summary>\n\nSome **markdown** here.\n\n- a list\n\n</details>\n\nafter\n";
        let (ast, html) = all(md);
        let d = &ast.children[0];
        assert_eq!(d.node_type, "HtmlElement");
        assert_eq!(d.info.as_deref(), Some("details"));
        assert_eq!(d.literal.as_deref(), Some("<details open=\"\"></details>"));
        let kinds: Vec<_> = d.children.iter().map(|c| c.node_type.as_str()).collect();
        assert_eq!(kinds, vec!["HtmlBlock", "Paragraph", "List"]);
        assert!(html.iter().any(|h| h.contains("<summary>More</summary>")), "{html:?}");
        assert_eq!(ast.children[1].node_type, "Paragraph", "content after </details> stays outside");
    }

    #[test]
    fn nested_same_tag_blocks() {
        let md = "<details>\n<summary>Outer</summary>\n\n<details>\n<summary>Inner</summary>\n\ninner body\n\n</details>\n\nouter body\n\n</details>\n";
        let (ast, _) = all(md);
        assert_eq!(elements(&ast), vec!["details", "details"]);
        let outer = &ast.children[0];
        assert!(outer.children.iter().any(|c| c.node_type == "HtmlElement"));
        assert_eq!(ast.children.len(), 1);
    }

    #[test]
    fn attributes_are_sanitized_on_wrappers() {
        let (ast, html) = all("<div align=\"center\" onclick=\"x()\" style=\"a>b\">\n\n**hi**\n\n</div>\n");
        assert_eq!(elements(&ast), vec!["div"]);
        assert!(html.iter().any(|h| h == "HtmlElement:<div align=\"center\"></div>"), "{html:?}");
        assert!(html.iter().all(|h| !h.contains("onclick")), "{html:?}");
    }

    #[test]
    fn removed_wrappers_keep_their_markdown() {
        let (ast, html) = all("<iframe src=\"https://x\">\n\nvisible text\n\n</iframe>\n");
        assert!(elements(&ast).is_empty());
        assert!(html.iter().all(|h| !h.contains("iframe")), "{html:?}");
        assert_eq!(ast.children[0].node_type, "Paragraph");
    }

    #[test]
    fn unmatched_and_void_tags_stay_separate_and_sanitized() {
        let (ast, html) = all("a <b>open only\n\nline<br>break <img src=x onerror=y> and </i> stray\n");
        assert!(elements(&ast).is_empty(), "{:?}", elements(&ast));
        assert!(html.iter().all(|h| !h.contains("onerror")), "{html:?}");
    }

    #[test]
    fn link_wrapping_markdown_keeps_safe_href_only() {
        let (ast, html) = all("<a href=\"other.md\">**go**</a> <a href=\"javascript:alert(1)\">x</a>\n");
        assert_eq!(elements(&ast), vec!["a", "a"]);
        assert!(html.iter().any(|h| h.contains("href=\"other.md\"")), "{html:?}");
        assert!(html.iter().all(|h| !h.contains("javascript")), "{html:?}");
    }

    #[test]
    fn html_in_containers_is_sanitized() {
        // Every container's HTML goes through the same final pass.
        let md = "- <img src=x onerror=a>\n\n> <img src=x onerror=b>\n\n> [!NOTE]\n> <img src=x onerror=c>\n\n| h |\n|---|\n| <img src=x onerror=d> |\n\n# <span onclick=e>H</span>\n\nx[^1]\n\n[^1]: <img src=x onerror=f>\n";
        let (_, html) = all(md);
        assert!(html.len() >= 6, "{html:?}");
        assert!(html.iter().all(|h| !h.contains("onerror") && !h.contains("onclick")), "{html:?}");
    }
}
