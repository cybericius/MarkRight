//! Rules for opening local files linked from documents.

use std::path::{Path, PathBuf};

/// Resolve a linked path and check it lies inside `root`.
///
/// Both sides are canonicalized, so `..` segments and symlinks pointing out of
/// the folder are caught.
///
/// # Errors
///
/// Returns an error if either path can't be resolved or the target is outside
/// `root`.
pub fn resolve_in_root(root: &Path, path: &Path) -> Result<PathBuf, String> {
    let root = std::fs::canonicalize(root).map_err(|e| e.to_string())?;
    let target = std::fs::canonicalize(path).map_err(|e| e.to_string())?;
    if target.starts_with(&root) {
        Ok(target)
    } else {
        Err(format!("Not opening a file outside the open folder: {}", target.display()))
    }
}

/// Find the file a wikilink target refers to, Obsidian-style.
///
/// `target` is the page part of `[[target]]` (a name or relative path, with or
/// without extension; `.md` is implied unless it has a Markdown extension). Lookup order:
/// relative to the linking document's folder, relative to `root`, then by file
/// name anywhere under `root` (case-insensitive, shortest path wins; hidden
/// entries skipped). The result is canonical and always inside `root`.
pub fn resolve_wikilink(root: &Path, from: &Path, target: &str) -> Option<PathBuf> {
    let target = target.trim().replace('\\', "/");
    if target.is_empty() {
        return None;
    }
    let mut candidates = vec![target.clone()];
    if !crate::files::is_markdown(Path::new(&target)) {
        candidates.push(format!("{target}.md"));
    }
    // A bare name like "Note.v2" has an "extension" that isn't one; try both.
    let in_root = |p: &Path| resolve_in_root(root, p).ok().filter(|p| p.is_file());

    for base in [from.parent(), Some(root)].into_iter().flatten() {
        for c in &candidates {
            if let Some(p) = in_root(&base.join(c)) {
                return Some(p);
            }
        }
    }

    let names: Vec<String> = candidates
        .iter()
        .filter_map(|c| Path::new(c).file_name()?.to_str().map(str::to_lowercase))
        .collect();
    let mut best: Option<PathBuf> = None;
    let mut stack = vec![root.to_path_buf()];
    let mut seen = 0usize;
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            seen += 1;
            if seen > 100_000 {
                return best.and_then(|p| in_root(&p));
            }
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if name.starts_with('.') {
                continue;
            }
            let path = entry.path();
            let Ok(ft) = entry.file_type() else { continue };
            if ft.is_dir() {
                stack.push(path);
            } else if names.contains(&name)
                && best.as_ref().is_none_or(|b| path.components().count() < b.components().count())
            {
                best = Some(path);
            }
        }
    }
    best.and_then(|p| in_root(&p))
}

/// Whether a linked file may be opened with the system default app.
///
/// Allow-list of passive document and media types. Anything else (scripts,
/// executables, shortcuts, macro-capable or browser-rendered formats) is only
/// revealed in the file manager, so a link in an untrusted document can't
/// launch a program.
pub fn opens_safely(path: &Path) -> bool {
    let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
        return false;
    };
    matches!(
        ext.to_ascii_lowercase().as_str(),
        "pdf" | "txt" | "csv" | "tsv" | "log" | "epub"
            | "png" | "jpg" | "jpeg" | "gif" | "webp" | "avif" | "bmp"
            | "mp3" | "wav" | "ogg" | "flac" | "m4a" | "mp4" | "webm" | "mov" | "mkv"
            | "docx" | "xlsx" | "pptx" | "odt" | "ods" | "odp"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_in_root_rejects_escapes() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("root");
        std::fs::create_dir_all(root.join("sub")).unwrap();
        std::fs::write(root.join("sub/a.pdf"), "x").unwrap();
        std::fs::write(dir.path().join("outside.pdf"), "x").unwrap();

        assert!(resolve_in_root(&root, &root.join("sub/a.pdf")).is_ok());
        assert!(resolve_in_root(&root, &root.join("sub/../sub/a.pdf")).is_ok());
        assert!(resolve_in_root(&root, &root.join("../outside.pdf")).is_err());
        assert!(resolve_in_root(&root, &dir.path().join("outside.pdf")).is_err());
        assert!(resolve_in_root(&root, &root.join("missing.pdf")).is_err());
        // A sibling folder sharing the name prefix is not inside the root
        std::fs::create_dir_all(dir.path().join("root2")).unwrap();
        std::fs::write(dir.path().join("root2/b.pdf"), "x").unwrap();
        assert!(resolve_in_root(&root, &dir.path().join("root2/b.pdf")).is_err());
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(dir.path().join("outside.pdf"), root.join("link.pdf")).unwrap();
            assert!(resolve_in_root(&root, &root.join("link.pdf")).is_err());
        }
    }

    #[test]
    fn resolve_wikilink_lookup_order() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        for f in ["notes/a.md", "notes/Sub Page.md", "Top.md", "deep/x/y/Top.md", "deep/Far.md", "img/pic one.png", ".hidden/Secret.md", "deep/Note.v2.md"] {
            let p = root.join(f);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(&p, "x").unwrap();
        }
        let from = root.join("notes/a.md");
        let r = |t: &str| resolve_wikilink(root, &from, t).map(|p| p.strip_prefix(root.canonicalize().unwrap()).unwrap().to_string_lossy().replace('\\', "/"));

        assert_eq!(r("Sub Page").as_deref(), Some("notes/Sub Page.md"), "same folder, .md implied");
        assert_eq!(r("sub page").as_deref(), Some("notes/Sub Page.md"), "case-insensitive by name");
        assert_eq!(r("Top").as_deref(), Some("Top.md"), "root-relative beats deeper match");
        assert_eq!(r("Far").as_deref(), Some("deep/Far.md"), "found by name anywhere");
        assert_eq!(r("deep/Far").as_deref(), Some("deep/Far.md"), "root-relative path");
        assert_eq!(r("pic one.png").as_deref(), Some("img/pic one.png"), "non-md files by name");
        assert_eq!(r("Note.v2").as_deref(), Some("deep/Note.v2.md"), "dotted name still gets .md");
        assert_eq!(r("Secret"), None, "hidden folders skipped");
        assert_eq!(r("../../etc/passwd"), None, "never outside root");
        assert_eq!(r("Missing"), None);
        assert_eq!(r(""), None);
    }

    #[test]
    fn opens_documents_and_media() {
        for p in ["a.pdf", "b.PDF", "c.csv", "d.docx", "e.png", "f.mp4"] {
            assert!(opens_safely(Path::new(p)), "{p}");
        }
    }

    #[test]
    fn never_opens_runnable_or_active_files() {
        for p in [
            "setup.exe", "run.bat", "run.cmd", "x.ps1", "x.sh", "x.desktop", "x.AppImage", "x.lnk",
            "x.jar", "x.vbs", "x.js", "x.msi", "x.scr", "x.html", "x.svg", "x.docm", "x.xls", "x.doc",
            "x.app", "Makefile", "noext", "x.pdf.exe",
        ] {
            assert!(!opens_safely(Path::new(p)), "{p}");
        }
    }
}
