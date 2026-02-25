use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

/// A file that contains search matches.
#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub path: PathBuf,
    pub name: String,
    pub matches: Vec<SearchMatch>,
}

/// A single line matching the search query.
#[derive(Debug, Clone, Serialize)]
pub struct SearchMatch {
    pub line_number: usize,
    pub line_text: String,
    pub col_start: usize,
    pub col_end: usize,
}

/// Maximum matching lines returned per file.
const MAX_MATCHES_PER_FILE: usize = 10;

/// Search all `.md` files under `root` for case-insensitive substring matches.
///
/// Returns up to `max_results` files with matches. Each file includes up to
/// [`MAX_MATCHES_PER_FILE`] matching lines with position information.
pub fn search_files(root: &Path, query: &str, max_results: usize) -> Vec<SearchResult> {
    if query.is_empty() {
        return Vec::new();
    }

    let query_lower = query.to_lowercase();
    let mut results = Vec::new();
    let mut paths = Vec::new();
    collect_md_files(root, &mut paths);

    for path in paths {
        if results.len() >= max_results {
            break;
        }

        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };

        let matches = find_matches(&content, &query_lower, query.len());
        if !matches.is_empty() {
            let name = path
                .file_name()
                .map_or_else(String::new, |n| n.to_string_lossy().to_string());
            results.push(SearchResult {
                path,
                name,
                matches,
            });
        }
    }

    results
}

/// Recursively collect all `.md` file paths, skipping hidden entries.
fn collect_md_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();
    entries.sort_by_key(fs::DirEntry::file_name);

    for entry in entries {
        let path = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if name_str.starts_with('.') {
            continue;
        }

        if path.is_dir() {
            collect_md_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "md") {
            out.push(path);
        }
    }
}

/// Find case-insensitive substring matches within file content.
fn find_matches(content: &str, query_lower: &str, query_len: usize) -> Vec<SearchMatch> {
    let mut matches = Vec::new();

    for (line_idx, line) in content.lines().enumerate() {
        if matches.len() >= MAX_MATCHES_PER_FILE {
            break;
        }

        let line_lower = line.to_lowercase();
        if let Some(col) = line_lower.find(query_lower) {
            matches.push(SearchMatch {
                line_number: line_idx + 1,
                line_text: line.to_string(),
                col_start: col,
                col_end: col + query_len,
            });
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_empty_query_returns_empty() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("test.md"), "# Hello").unwrap();
        let results = search_files(dir.path(), "", 50);
        assert!(results.is_empty());
    }

    #[test]
    fn test_no_matches() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("test.md"), "# Hello world").unwrap();
        let results = search_files(dir.path(), "zebra", 50);
        assert!(results.is_empty());
    }

    #[test]
    fn test_case_insensitive_match() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("test.md"), "# Hello World\nSome text").unwrap();
        let results = search_files(dir.path(), "hello", 50);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "test.md");
        assert_eq!(results[0].matches.len(), 1);
        assert_eq!(results[0].matches[0].line_number, 1);
        assert_eq!(results[0].matches[0].col_start, 2);
        assert_eq!(results[0].matches[0].col_end, 7);
    }

    #[test]
    fn test_multiple_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("a.md"), "# Alpha\nfoo bar").unwrap();
        fs::write(dir.path().join("b.md"), "# Beta\nbaz qux").unwrap();
        fs::write(dir.path().join("c.txt"), "foo bar").unwrap(); // not .md

        let results = search_files(dir.path(), "foo", 50);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "a.md");
    }

    #[test]
    fn test_nested_directory() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("sub");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("deep.md"), "nested content here").unwrap();

        let results = search_files(dir.path(), "nested", 50);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "deep.md");
    }

    #[test]
    fn test_skips_hidden() {
        let dir = tempfile::tempdir().unwrap();
        let hidden = dir.path().join(".hidden");
        fs::create_dir(&hidden).unwrap();
        fs::write(hidden.join("secret.md"), "secret content").unwrap();

        let results = search_files(dir.path(), "secret", 50);
        assert!(results.is_empty());
    }

    #[test]
    fn test_max_results_limit() {
        let dir = tempfile::tempdir().unwrap();
        for i in 0..10 {
            fs::write(dir.path().join(format!("file{i}.md")), "match this").unwrap();
        }

        let results = search_files(dir.path(), "match", 3);
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_multiple_matches_per_file() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("test.md"),
            "foo line one\nbar line two\nfoo line three",
        )
        .unwrap();

        let results = search_files(dir.path(), "foo", 50);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].matches.len(), 2);
        assert_eq!(results[0].matches[0].line_number, 1);
        assert_eq!(results[0].matches[1].line_number, 3);
    }
}
