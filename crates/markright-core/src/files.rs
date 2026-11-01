//! Which files the app treats as Markdown documents.
//!
//! The single list every Rust caller uses (tree, search, wikilinks). The
//! frontend keeps the same list in `src/lib/markdown.ts`, and the OS file
//! associations in `src-tauri/tauri.conf.json`; a test below keeps all three
//! in sync.

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
        Cow::Owned(strip_mdx_esm(body))
    } else {
        Cow::Borrowed(body)
    }
}

/// Remove MDX's top-level ESM statements (`import …` / `export …`) so they
/// don't render as paragraphs. Lines inside fenced code blocks are kept, and
/// a statement continues until a line ending with `;` or a blank line.
/// JSX components need nothing here: unknown tags are dropped by the HTML
/// sanitizer and the Markdown inside them is kept.
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

        let mut rust: Vec<String> = MARKDOWN_EXTENSIONS
            .iter()
            .map(|s| (*s).to_string())
            .collect();
        rust.sort();
        ts_exts.sort();
        conf_exts.sort();
        assert_eq!(ts_exts, rust, "src/lib/markdown.ts");
        assert_eq!(conf_exts, rust, "tauri.conf.json fileAssociations");
    }
}
