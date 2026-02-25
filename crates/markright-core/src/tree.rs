use serde::Serialize;
use std::path::{Path, PathBuf};

/// Represents a file or directory in the navigation tree.
#[derive(Debug, Serialize)]
pub struct TreeNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub children: Vec<TreeNode>,
}

/// Recursively scan a directory and build a tree of `.md` files.
///
/// # Errors
///
/// Returns an error if the directory cannot be read.
pub fn build_tree(root: &Path) -> std::io::Result<Vec<TreeNode>> {
    let mut nodes = Vec::new();

    if !root.is_dir() {
        return Ok(nodes);
    }

    let mut entries: Vec<_> = std::fs::read_dir(root)?
        .filter_map(std::result::Result::ok)
        .collect();

    entries.sort_by_key(std::fs::DirEntry::file_name);

    for entry in entries {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        // Skip hidden files/dirs
        if name.starts_with('.') {
            continue;
        }

        if path.is_dir() {
            let children = build_tree(&path)?;
            // Only include directories that contain markdown files
            if !children.is_empty() {
                nodes.push(TreeNode {
                    name,
                    path,
                    is_dir: true,
                    children,
                });
            }
        } else if path.extension().is_some_and(|ext| ext == "md") {
            nodes.push(TreeNode {
                name,
                path,
                is_dir: false,
                children: Vec::new(),
            });
        }
    }

    Ok(nodes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_build_tree_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let tree = build_tree(dir.path()).unwrap();
        assert!(tree.is_empty());
    }

    #[test]
    fn test_build_tree_with_md_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("readme.md"), "# Hello").unwrap();
        fs::write(dir.path().join("notes.txt"), "not markdown").unwrap();

        let tree = build_tree(dir.path()).unwrap();
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].name, "readme.md");
    }
}
