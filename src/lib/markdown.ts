/**
 * File extensions opened as Markdown. Must match MARKDOWN_EXTENSIONS in
 * crates/markright-core/src/files.rs and the tauri.conf.json file
 * associations (a Rust test checks all three).
 */
export const MARKDOWN_EXTENSIONS = ["md", "markdown", "mdown", "mkd", "mdx"];

/** Matches a path ending in a Markdown extension (case-insensitive). */
export const MARKDOWN_EXT = new RegExp(`\\.(${MARKDOWN_EXTENSIONS.join("|")})$`, "i");
