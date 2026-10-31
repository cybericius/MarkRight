//! Sanitization of untrusted document content before it reaches the webview.
//!
//! Documents are opened from arbitrary folders, so raw HTML and link targets
//! are untrusted. Anything that could run script in the app window (which has
//! IPC access) must be stripped here, at AST build time.

use std::sync::LazyLock;

static HTML_CLEANER: LazyLock<ammonia::Builder<'static>> = LazyLock::new(|| {
    let mut b = ammonia::Builder::default();
    // `<p align="center">` is common in READMEs and harmless.
    b.add_generic_attributes(["align"]);
    // `<details open>` starts expanded; harmless.
    b.add_tag_attributes("details", ["open"]);
    b
});

/// Strip scripts, event handlers, and dangerous URLs from an HTML fragment.
pub fn sanitize_html(html: &str) -> String {
    HTML_CLEANER.clean(html).to_string()
}

/// Which element a URL is used by; images may also use `data:image/...`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UrlKind {
    Link,
    Image,
}

/// Return the URL if it is safe to put in `href`/`src`, otherwise `None`.
///
/// Relative URLs, fragments and absolute paths pass through; absolute URLs
/// must use an allowed scheme.
pub fn safe_url(url: &str, kind: UrlKind) -> Option<String> {
    // Browsers ignore leading whitespace/control chars and embedded tab/CR/LF
    // when parsing a scheme ("java\tscript:"), so normalize the same way.
    let normalized: String = url
        .trim_start_matches(|c: char| c <= ' ')
        .chars()
        .filter(|c| !matches!(c, '\t' | '\n' | '\r'))
        .collect();

    let Some(scheme) = scheme_of(&normalized) else {
        return Some(url.to_string());
    };
    let scheme = scheme.to_ascii_lowercase();
    let allowed = match scheme.as_str() {
        "http" | "https" | "mailto" => true,
        "data" => kind == UrlKind::Image && normalized[5..].to_ascii_lowercase().starts_with("image/"),
        _ => false,
    };
    allowed.then(|| url.to_string())
}

/// The URL scheme, if the string has one. A single letter followed by `:` is a
/// Windows drive (`C:/img.png`), not a scheme.
fn scheme_of(url: &str) -> Option<&str> {
    let end = url.find([':', '/', '?', '#'])?;
    if !url[end..].starts_with(':') {
        return None;
    }
    let scheme = &url[..end];
    let mut chars = scheme.chars();
    let valid = chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'));
    (valid && scheme.len() > 1).then_some(scheme)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_script_and_event_handlers() {
        let out = sanitize_html(r#"<img src="x.png" onerror="alert(1)"><script>alert(2)</script>"#);
        assert!(!out.contains("onerror"), "{out}");
        assert!(!out.contains("script"), "{out}");
        assert!(out.contains(r#"src="x.png""#), "{out}");
    }

    #[test]
    fn strips_javascript_href_in_html() {
        let out = sanitize_html(r#"<a href="javascript:alert(1)">x</a>"#);
        assert!(!out.contains("javascript"), "{out}");
    }

    #[test]
    fn strips_iframe_object_svg_script() {
        for html in [
            r#"<iframe src="https://evil"></iframe>"#,
            r#"<object data="x.swf"></object>"#,
            "<svg><script>alert(1)</script></svg>",
            r#"<div style="background:url(javascript:alert(1))">x</div>"#,
        ] {
            let out = sanitize_html(html);
            assert!(!out.contains("iframe") && !out.contains("object") && !out.contains("script") && !out.contains("javascript"), "{html} -> {out}");
        }
    }

    #[test]
    fn keeps_harmless_html() {
        let out = sanitize_html(r#"<p align="center"><img src="logo.png" width="100" alt="Logo"></p><details><summary>More</summary>body</details><kbd>Ctrl</kbd>"#);
        for keep in [r#"align="center""#, r#"src="logo.png""#, r#"width="100""#, "<details>", "<summary>", "<kbd>"] {
            assert!(out.contains(keep), "missing {keep} in {out}");
        }
    }

    #[test]
    fn rejects_dangerous_url_schemes() {
        for url in ["javascript:alert(1)", "JavaScript:alert(1)", " javascript:x", "java\tscript:x", "vbscript:x", "file:///etc/passwd", "data:text/html,<script>x</script>"] {
            assert_eq!(safe_url(url, UrlKind::Link), None, "{url:?}");
        }
        assert_eq!(safe_url("data:text/html,x", UrlKind::Image), None);
    }

    #[test]
    fn keeps_safe_urls() {
        for url in ["https://example.com", "http://x", "mailto:a@b.c", "#heading", "other.md", "../img/a.png", "/abs/path.png", "C:/docs/a.png", "dir/file:name.md", "?q=1"] {
            assert_eq!(safe_url(url, UrlKind::Link).as_deref(), Some(url), "{url:?}");
        }
        assert!(safe_url("data:image/png;base64,AAAA", UrlKind::Image).is_some());
    }
}
