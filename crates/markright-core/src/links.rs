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
