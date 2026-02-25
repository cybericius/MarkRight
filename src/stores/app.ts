import { createSignal } from "solid-js";
import type {
  TreeNode,
  MdNode,
  TocEntry,
  Frontmatter,
  SearchResult,
} from "../lib/types";

export const [tree, setTree] = createSignal<TreeNode[]>([]);
export const [rootFolder, setRootFolder] = createSignal<string | null>(null);
export const [currentPath, setCurrentPath] = createSignal<string | null>(null);
export const [ast, setAst] = createSignal<MdNode | null>(null);
export const [toc, setToc] = createSignal<TocEntry[]>([]);
export const [frontmatter, setFrontmatter] = createSignal<Frontmatter | null>(null);
export const [activeTocId, setActiveTocId] = createSignal<string | null>(null);

// Search
export const [searchMode, setSearchMode] = createSignal(false);
export const [searchQuery, setSearchQuery] = createSignal("");
export const [searchResults, setSearchResults] = createSignal<SearchResult[]>([]);

// Panel layout
export const [leftPanelWidth, setLeftPanelWidth] = createSignal(256);
export const [rightPanelWidth, setRightPanelWidth] = createSignal(224);
export const [showLeftPanel, setShowLeftPanel] = createSignal(true);
export const [showRightPanel, setShowRightPanel] = createSignal(true);
