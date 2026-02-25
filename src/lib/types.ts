/** Serialized Markdown AST node from Rust. */
export interface MdNode {
  type: string;
  children: MdNode[];
  level?: number;
  id?: string;
  literal?: string;
  url?: string;
  title?: string;
  info?: string;
  highlighted_html?: string;
  list_type?: string;
  start?: number;
  tight?: boolean;
  checked?: boolean;
  header?: boolean;
  alignments?: string[];
}

/** File/directory entry in the navigation tree. */
export interface TreeNode {
  name: string;
  path: string;
  is_dir: boolean;
  children: TreeNode[];
}

/** Table of Contents entry. */
export interface TocEntry {
  level: number;
  text: string;
  id: string;
}

/** Frontmatter parsed from YAML header. */
export interface Frontmatter {
  title: string | null;
  fields: Record<string, string>;
}

/** Response from the get_document command. */
export interface DocumentResponse {
  ast: MdNode;
  toc: TocEntry[];
  frontmatter: Frontmatter | null;
}

/** A file containing search matches. */
export interface SearchResult {
  path: string;
  name: string;
  matches: SearchMatch[];
}

/** A single line matching the search query. */
export interface SearchMatch {
  line_number: number;
  line_text: string;
  col_start: number;
  col_end: number;
}
