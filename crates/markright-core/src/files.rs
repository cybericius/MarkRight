//! Which files the app treats as Markdown documents.
//!
//! The single list every Rust caller uses (tree, search, wikilinks). The
//! frontend keeps the same list in `src/lib/markdown.ts`, the OS file
//! associations in `src-tauri/tauri.conf.json`, and the Windows installer
//! hooks in `src-tauri/windows/hooks.nsh`; a test below keeps them in sync.

use std::borrow::Cow;
use std::path::Path;

/// File extensions opened as Markdown, lowercase, without the dot.
pub const MARKDOWN_EXTENSIONS: &[&str] = &["md", "markdown", "mdown", "mkd", "mdx"];

/// True when `path` has a Markdown extension (case-insensitive).
#[must_use]
pub fn is_markdown(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()).is_some_and(|e| {
        MARKDOWN_EXTENSIONS
            .iter()
            .any(|m| e.eq_ignore_ascii_case(m))
    })
}

/// True when `path` is an MDX document.
#[must_use]
pub fn is_mdx(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("mdx"))
}

/// The part of a document's body to render: MDX gets its ESM statements
/// removed, every other Markdown file is used as is.
#[must_use]
pub fn render_body<'a>(path: &Path, body: &'a str) -> Cow<'a, str> {
    if is_mdx(path) {
        Cow::Owned(strip_jsx_expressions(&strip_mdx_esm(body)))
    } else {
        Cow::Borrowed(body)
    }
}

/// Remove MDX's top-level ESM statements (`import …` / `export …`) so they
/// don't render as paragraphs. Lines inside fenced code blocks are kept, and
/// a statement continues until a line ending with `;` or a blank line.
/// JSX tags are handled by [`strip_jsx_expressions`] and the HTML sanitizer.
#[must_use]
pub fn strip_mdx_esm(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let mut fence: Option<String> = None;
    let mut in_esm = false;

    for line in body.split_inclusive('\n') {
        let trimmed = line.trim_end_matches(['\n', '\r']);

        if let Some(f) = &fence {
            if trimmed.trim_start().starts_with(f.as_str()) {
                fence = None;
            }
            out.push_str(line);
            continue;
        }

        if in_esm {
            if trimmed.trim().is_empty() || trimmed.trim_end().ends_with(';') {
                in_esm = false;
            }
            if trimmed.trim().is_empty() {
                out.push_str(line);
            }
            continue;
        }

        let start = trimmed.trim_start();
        if start.starts_with("```") || start.starts_with("~~~") {
            fence = Some(start[..3].to_string());
            out.push_str(line);
            continue;
        }

        // Only unindented statements: MDX treats indented ones as content.
        if trimmed.starts_with("import ") || trimmed.starts_with("export ") {
            let t = trimmed.trim_end();
            // A one-liner ends with `;`, or is a complete `import x from '…'`.
            let complete = t.ends_with(';')
                || (t.starts_with("import ") && (t.ends_with('\'') || t.ends_with('"')));
            in_esm = !complete;
            continue;
        }

        out.push_str(line);
    }
    out
}

/// Make MDX's JSX tags plain HTML so they are dropped like other unknown tags.
///
/// A JSX attribute can hold a `{…}` expression (`<Chart data={[1, 2]} />`),
/// which is not valid HTML, so the tag would render as text. Expression
/// values become `""` and spreads (`{...props}`) are removed. MDX comments
/// (`{/* … */}`) are removed. Code blocks and code spans are left alone.
#[must_use]
pub fn strip_jsx_expressions(body: &str) -> String {
    let b = body.as_bytes();
    let mut out = String::with_capacity(body.len());
    let mut fence: Option<&str> = None;
    let mut i = 0;

    while i < b.len() {
        // Fenced code blocks, checked at each line start
        if i == 0 || b[i - 1] == b'\n' {
            let end = body[i..].find('\n').map_or(b.len(), |n| i + n + 1);
            let start = body[i..end].trim_start();
            if let Some(f) = fence {
                if start.starts_with(f) {
                    fence = None;
                }
                out.push_str(&body[i..end]);
                i = end;
                continue;
            }
            if start.starts_with("```") || start.starts_with("~~~") {
                fence = Some(&start[..3]);
                out.push_str(&body[i..end]);
                i = end;
                continue;
            }
        }

        match b[i] {
            // Code span: copy through the closing run of the same length
            b'`' => {
                let run = b[i..].iter().take_while(|&&c| c == b'`').count();
                let ticks = &body[i..i + run];
                let close = body[i + run..]
                    .match_indices(ticks)
                    .find(|&(p, _)| body.as_bytes().get(i + run + p + run) != Some(&b'`'))
                    .map(|(p, _)| i + run + p + run);
                let end = close.unwrap_or(i + run);
                out.push_str(&body[i..end]);
                i = end;
            }
            b'{' if body[i..].starts_with("{/*") => {
                if let Some(p) = body[i..].find("*/}") {
                    i += p + 3;
                } else {
                    out.push('{');
                    i += 1;
                }
            }
            b'<' if b.get(i + 1).is_some_and(u8::is_ascii_alphabetic) => {
                if let Some((tag, end)) = jsx_tag(body, i) {
                    out.push_str(&tag);
                    i = end;
                } else {
                    out.push('<');
                    i += 1;
                }
            }
            _ => {
                let ch = body[i..].chars().next().unwrap_or_default();
                out.push(ch);
                i += ch.len_utf8();
            }
        }
    }
    out
}

/// Parse the tag starting at `start` (a `<`). Returns the tag with its `{…}`
/// expressions replaced and the index after its `>`, or `None` when the tag
/// has no expression or doesn't close.
fn jsx_tag(body: &str, start: usize) -> Option<(String, usize)> {
    let b = body.as_bytes();
    let mut out = String::new();
    let mut changed = false;
    let mut quote: Option<u8> = None;
    let mut i = start;
    while i < b.len() {
        let c = b[i];
        // A blank line ends the attempt: this was not a tag
        if c == b'\n' && body[i + 1..].trim_start_matches([' ', '\t', '\r']).starts_with('\n') {
            return None;
        }
        if let Some(q) = quote {
            if c == q {
                quote = None;
            }
        } else {
            match c {
                b'"' | b'\'' => quote = Some(c),
                b'>' => {
                    out.push('>');
                    return changed.then_some((out, i + 1));
                }
                b'{' => {
                    let end = expression_end(body, i)?;
                    if out.trim_end().ends_with('=') {
                        out.push_str("\"\"");
                    }
                    changed = true;
                    i = end;
                    continue;
                }
                _ => {}
            }
        }
        let ch = body[i..].chars().next()?;
        out.push(ch);
        i += ch.len_utf8();
    }
    None
}

/// Index after the `}` that closes the `{` at `start`, skipping strings.
fn expression_end(body: &str, start: usize) -> Option<usize> {
    let b = body.as_bytes();
    let mut depth = 0usize;
    let mut quote: Option<u8> = None;
    let mut i = start;
    while i < b.len() {
        let c = b[i];
        if let Some(q) = quote {
            if c == b'\\' {
                i += 1;
            } else if c == q {
                quote = None;
            }
        } else {
            match c {
                b'"' | b'\'' | b'`' => quote = Some(c),
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i + 1);
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_extensions_case_insensitive() {
        for p in [
            "a.md",
            "a.MD",
            "a.markdown",
            "a.Mdown",
            "a.mkd",
            "a.mdx",
            "a.MDX",
            "dir.v2/x.md",
        ] {
            assert!(is_markdown(Path::new(p)), "{p}");
        }
        for p in ["a.txt", "a", "a.md.txt", "a.mdxx", ".md", "a.pdf"] {
            assert!(!is_markdown(Path::new(p)), "{p}");
        }
    }

    #[test]
    fn mdx_detection() {
        assert!(is_mdx(Path::new("x.mdx")));
        assert!(is_mdx(Path::new("x.MDX")));
        assert!(!is_mdx(Path::new("x.md")));
    }

    #[test]
    fn strips_imports_and_exports() {
        let src = "import { Tabs } from '@site/tabs'\nimport Foo from \"./foo\";\nexport const meta = { title: 'x' };\n\n# Title\n\nText\n";
        assert_eq!(strip_mdx_esm(src), "\n# Title\n\nText\n");
    }

    #[test]
    fn multiline_statement_until_semicolon() {
        let src = "export const meta = {\n  title: 'x',\n};\n# H\n";
        assert_eq!(strip_mdx_esm(src), "# H\n");
    }

    #[test]
    fn multiline_statement_until_blank_line() {
        let src = "import {\n  A,\n  B,\n} from './x'\n\n# H\n";
        assert_eq!(strip_mdx_esm(src), "\n# H\n");
    }

    #[test]
    fn keeps_code_fences_and_prose() {
        let src = "```js\nimport x from 'y'\nexport default x;\n```\n\nWe import data daily.\n  import indented stays\n";
        assert_eq!(strip_mdx_esm(src), src);
    }

    #[test]
    fn render_body_only_changes_mdx() {
        let src = "import A from 'a'\n# H\n";
        assert_eq!(render_body(Path::new("x.md"), src), src);
        assert_eq!(render_body(Path::new("x.mdx"), src), "# H\n");
    }

    #[test]
    fn tilde_fence_kept() {
        let src = "~~~\nexport A;\n~~~\n";
        assert_eq!(strip_mdx_esm(src), src);
    }

    /// A typical Docusaurus page: ESM gone, JSX wrappers gone, Markdown kept.
    #[test]
    fn mdx_page_renders_its_markdown() {
        fn text(n: &crate::ast::MdNode, out: &mut String) {
            if let Some(l) = &n.literal {
                out.push_str(l);
                out.push('\n');
            }
            for c in &n.children {
                text(c, out);
            }
        }
        let src = "import Tabs from '@theme/Tabs';\nimport TabItem from '@theme/TabItem';\n\n# Install\n\n<Tabs>\n<TabItem value=\"deb\">\n\nRun **dpkg**.\n\n</TabItem>\n</Tabs>\n";
        let ast = crate::ast::serialize_ast(&render_body(Path::new("a.mdx"), src));
        let mut all = String::new();
        text(&ast, &mut all);
        assert!(!all.contains("import"), "{all}");
        assert!(!all.contains("Tabs") && !all.contains("TabItem"), "{all}");
        assert!(all.contains("Install") && all.contains("dpkg"), "{all}");
    }

    fn rendered_text(mdx: &str) -> String {
        // Text the reader sees as text; HTML nodes render as markup
        fn text(n: &crate::ast::MdNode) -> String {
            let own = if n.node_type.starts_with("Html") {
                ""
            } else {
                n.literal.as_deref().unwrap_or("")
            };
            own.to_string() + &n.children.iter().map(text).collect::<String>()
        }
        let body = render_body(Path::new("x.mdx"), mdx);
        text(&crate::ast::serialize_ast(&body))
    }

    #[test]
    fn jsx_expression_attributes_are_dropped() {
        for src in [
            "<Chart data={[1, 2, 3]} />\n",
            "Version <Badge color={\"blue\"}>0.3.1</Badge> here\n",
            "<Tabs groupId=\"os\" values={[{label: 'A', value: 'a'}, {label: 'B > C', value: 'b'}]}>\n\nBody\n\n</Tabs>\n",
            "<Tabs\n  groupId=\"os\"\n  values={[\n    {label: 'Linux', value: 'linux'},\n  ]}>\n\nBody\n\n</Tabs>\n",
            "<Card {...props} title={`Hi ${name}`} />\n",
            "<div style={{color: 'red'}}>styled</div>\n",
            "Text {/* an MDX comment */} after\n",
        ] {
            let all = rendered_text(src);
            for leak in ["{", "}", "Chart", "Badge", "Tabs", "Card", "props", "comment", "<"] {
                assert!(!all.contains(leak), "{leak:?} leaked from {src:?}: {all:?}");
            }
        }
        assert!(rendered_text("Version <Badge color={\"blue\"}>0.3.1</Badge> here\n").contains("Version 0.3.1 here"));
        assert!(rendered_text("<Tabs values={[1]}>\n\nBody **bold**\n\n</Tabs>\n").contains("Body bold"));
    }

    #[test]
    fn jsx_expressions_kept_in_code_and_text() {
        // Code blocks, code spans, math and plain braces are not JSX
        let src = "```jsx\n<Chart data={[1, 2]} />\n{/* keep */}\n```\n\nUse `<Chart data={x} />` and `{/* c */}`.\n\nMath $\\frac{a}{b}$ and a < b {c} > d.\n";
        let out = strip_jsx_expressions(src);
        assert_eq!(out, src);
        // .md files are never touched
        assert_eq!(render_body(Path::new("x.md"), "<Chart data={[1]} />"), "<Chart data={[1]} />");
    }

    #[test]
    fn unclosed_jsx_is_left_alone() {
        for src in ["<Chart data={[1, 2\n\nnext paragraph\n", "<Chart data={1}\n\nno close\n", "it's <b and {x}\n"] {
            assert_eq!(strip_jsx_expressions(src), src, "{src:?}");
        }
    }

    /// The frontend list and the OS file associations must match this one.
    #[test]
    fn extension_lists_in_sync() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

        let ts = std::fs::read_to_string(root.join("src/lib/markdown.ts")).unwrap();
        let line = ts
            .lines()
            .find(|l| l.contains("MARKDOWN_EXTENSIONS ="))
            .expect("list in markdown.ts");
        let mut ts_exts: Vec<String> = line
            .split('"')
            .skip(1)
            .step_by(2)
            .map(str::to_string)
            .collect();

        let conf: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(root.join("src-tauri/tauri.conf.json")).unwrap(),
        )
        .unwrap();
        let mut conf_exts: Vec<String> = conf["bundle"]["fileAssociations"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|a| {
                a["ext"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|e| e.as_str().unwrap().to_string())
            })
            .collect();

        // Windows installer hooks: `!insertmacro ${_MACRO} "md"` lines
        let nsh = std::fs::read_to_string(root.join("src-tauri/windows/hooks.nsh")).unwrap();
        let mut nsh_exts: Vec<String> = nsh
            .lines()
            .filter_map(|l| l.trim().strip_prefix("!insertmacro ${_MACRO} \""))
            .map(|rest| rest.trim_end_matches('"').to_string())
            .collect();

        let mut rust: Vec<String> = MARKDOWN_EXTENSIONS
            .iter()
            .map(|s| (*s).to_string())
            .collect();
        rust.sort();
        ts_exts.sort();
        conf_exts.sort();
        nsh_exts.sort();
        assert_eq!(ts_exts, rust, "src/lib/markdown.ts");
        assert_eq!(conf_exts, rust, "tauri.conf.json fileAssociations");
        assert_eq!(nsh_exts, rust, "src-tauri/windows/hooks.nsh MR_EACH_EXT");
    }
}
