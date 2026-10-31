//! GitHub alerts and Obsidian callouts.
//!
//! Both use a blockquote whose first line is `[!type]`. GitHub has five types;
//! Obsidian adds more types, aliases, a custom title after the marker, and a
//! fold marker (`[!tip]-` collapsed, `[!tip]+` expanded). comrak's `alerts`
//! extension only knows GitHub's five, so both are recognized here instead.

use crate::ast::MdNode;

/// Canonical kind for a `[!type]` marker. Unknown types render as a note,
/// as in Obsidian.
fn canonical_kind(marker: &str) -> &'static str {
    match marker.to_ascii_lowercase().as_str() {
        "abstract" | "summary" | "tldr" => "abstract",
        "info" => "info",
        "todo" => "todo",
        "tip" | "hint" => "tip",
        "important" => "important",
        "success" | "check" | "done" => "success",
        "question" | "help" | "faq" => "question",
        "warning" | "attention" => "warning",
        // GitHub's CAUTION is its red alert; Obsidian treats it as a warning.
        // GitHub files are more common, so it keeps GitHub's meaning.
        "caution" => "caution",
        "failure" | "fail" | "missing" => "failure",
        "danger" | "error" => "danger",
        "bug" => "bug",
        "example" => "example",
        "quote" | "cite" => "quote",
        _ => "note",
    }
}

/// Default title: the marker as written, capitalized (`[!faq]` → "Faq").
fn default_title(marker: &str) -> String {
    let lower = marker.to_ascii_lowercase();
    let mut chars = lower.chars();
    chars
        .next()
        .map(|c| c.to_ascii_uppercase().to_string() + chars.as_str())
        .unwrap_or_default()
}

/// Parse `[!type]`, an optional fold sign, and the rest of the line.
fn parse_marker(text: &str) -> Option<(&str, Option<&'static str>, &str)> {
    let rest = text.strip_prefix("[!")?;
    let end = rest.find(']')?;
    let marker = &rest[..end];
    if marker.is_empty() || !marker.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        return None;
    }
    let mut after = &rest[end + 1..];
    let fold = match after.chars().next() {
        Some('-') => Some("closed"),
        Some('+') => Some("open"),
        _ => None,
    };
    if fold.is_some() {
        after = &after[1..];
    }
    // The marker must be followed by whitespace or the end of the line.
    if !after.is_empty() && !after.starts_with(char::is_whitespace) {
        return None;
    }
    Some((marker, fold, after.trim_start()))
}

/// Turn a `BlockQuote` whose first line is an alert marker into an `Alert`.
///
/// The `Alert` carries the kind in `info`, the default title in `title`, the
/// fold state in `fold`, and an `AlertTitle` first child when the line has a
/// custom title (kept as inline nodes so formatting survives).
pub(crate) fn convert_blockquote(node: &mut MdNode) {
    let Some(para) = node.children.first_mut().filter(|p| p.node_type == "Paragraph") else {
        return;
    };
    let Some(first) = para.children.first().filter(|t| t.node_type == "Text") else {
        return;
    };
    let Some((marker, fold, rest)) = first.literal.as_deref().and_then(parse_marker) else {
        return;
    };
    let kind = canonical_kind(marker);
    let title = default_title(marker);
    let rest = rest.to_string();

    // Split the paragraph at the end of the marker line.
    let break_at = para
        .children
        .iter()
        .position(|n| n.node_type == "SoftBreak" || n.node_type == "LineBreak")
        .unwrap_or(para.children.len());
    let mut body: Vec<MdNode> = para.children.split_off(break_at);
    if !body.is_empty() {
        body.remove(0); // the line break itself
    }
    let mut title_nodes: Vec<MdNode> = para.children.drain(1..).collect();
    if !rest.is_empty() {
        let mut text = MdNode::new("Text");
        text.literal = Some(rest);
        title_nodes.insert(0, text);
    }
    if let Some(last) = title_nodes.last_mut().filter(|n| n.node_type == "Text") {
        let trimmed = last.literal.as_deref().unwrap_or("").trim_end().to_string();
        last.literal = Some(trimmed);
    }
    title_nodes.retain(|n| n.node_type != "Text" || n.literal.as_deref().is_some_and(|s| !s.is_empty()));

    para.children = body;
    let mut children = std::mem::take(&mut node.children);
    if children.first().is_some_and(|p| p.children.is_empty()) {
        children.remove(0);
    }
    if !title_nodes.is_empty() {
        let mut t = MdNode::new("AlertTitle");
        t.children = title_nodes;
        children.insert(0, t);
    }

    node.node_type = "Alert".to_string();
    node.info = Some(kind.to_string());
    node.title = Some(title);
    node.fold = fold.map(str::to_string);
    node.children = children;
}

#[cfg(test)]
mod tests {
    use crate::ast::{MdNode, serialize_ast};

    fn first(md: &str) -> MdNode {
        serialize_ast(md).children.into_iter().next().unwrap()
    }

    fn text(n: &MdNode) -> String {
        n.literal.clone().unwrap_or_default() + &n.children.iter().map(text).collect::<String>()
    }

    #[test]
    fn github_alert_types() {
        for (md, kind, title) in [
            ("> [!NOTE]\n> x", "note", "Note"),
            ("> [!TIP]\n> x", "tip", "Tip"),
            ("> [!IMPORTANT]\n> x", "important", "Important"),
            ("> [!WARNING]\n> x", "warning", "Warning"),
            ("> [!CAUTION]\n> x", "caution", "Caution"),
        ] {
            let a = first(md);
            assert_eq!(a.node_type, "Alert", "{md}");
            assert_eq!(a.info.as_deref(), Some(kind));
            assert_eq!(a.title.as_deref(), Some(title));
            assert_eq!(a.fold, None);
            assert_eq!(a.children.len(), 1, "body paragraph only: {md}");
            assert_eq!(text(&a.children[0]), "x");
        }
    }

    #[test]
    fn obsidian_types_aliases_and_unknown() {
        for (md, kind, title) in [
            ("> [!info]\n> x", "info", "Info"),
            ("> [!faq]\n> x", "question", "Faq"),
            ("> [!tldr]\n> x", "abstract", "Tldr"),
            ("> [!error]\n> x", "danger", "Error"),
            ("> [!custom-thing]\n> x", "note", "Custom-thing"),
        ] {
            let a = first(md);
            assert_eq!((a.info.as_deref(), a.title.as_deref()), (Some(kind), Some(title)), "{md}");
        }
    }

    #[test]
    fn custom_title_keeps_formatting_and_fold() {
        let a = first("> [!tip]- My *fancy* title\n> body **x**\n");
        assert_eq!(a.fold.as_deref(), Some("closed"));
        assert_eq!(a.children[0].node_type, "AlertTitle");
        assert_eq!(text(&a.children[0]), "My fancy title");
        assert!(a.children[0].children.iter().any(|n| n.node_type == "Emph"));
        assert_eq!(text(&a.children[1]), "body x");

        assert_eq!(first("> [!note]+\n> x").fold.as_deref(), Some("open"));
    }

    #[test]
    fn marker_only_and_multi_paragraph() {
        let a = first("> [!WARNING]\n");
        assert_eq!(a.node_type, "Alert");
        assert!(a.children.is_empty());

        let a = first("> [!NOTE]\n> one\n>\n> two\n");
        assert_eq!(a.children.iter().map(text).collect::<Vec<_>>(), vec!["one", "two"]);
    }

    #[test]
    fn plain_blockquotes_stay_blockquotes() {
        for md in ["> just a quote", "> see [!NOTE] later", "> [!NOTE]x\n> y", "> [! NOTE]\n> y", "> [!]\n> y", "> # [!NOTE]\n> y"] {
            assert_eq!(first(md).node_type, "BlockQuote", "{md}");
        }
    }

    #[test]
    fn nested_alert_in_blockquote() {
        let q = first("> outer\n>\n> > [!TIP]\n> > inner\n");
        assert_eq!(q.node_type, "BlockQuote");
        assert!(q.children.iter().any(|n| n.node_type == "Alert"));
    }
}
