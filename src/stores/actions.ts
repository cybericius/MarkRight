import { openFolder as ipcOpenFolder, getDocument, search as ipcSearch } from "../lib/tauri";
import {
  setTree,
  setRootFolder,
  setCurrentPath,
  setAst,
  setToc,
  setFrontmatter,
  setActiveTocId,
  setSearchQuery,
  setSearchResults,
  setSearchMode,
  searchMode,
} from "./app";

export async function openFolder(path: string): Promise<void> {
  const tree = await ipcOpenFolder(path);
  setTree(tree);
  setRootFolder(path);
  // Clear document state
  setCurrentPath(null);
  setAst(null);
  setToc([]);
  setFrontmatter(null);
  setActiveTocId(null);
}

export async function openDocument(path: string): Promise<void> {
  const doc = await getDocument(path);
  setCurrentPath(path);
  setAst(doc.ast);
  setToc(doc.toc);
  setFrontmatter(doc.frontmatter);
  setActiveTocId(null);
}

let searchTimer: ReturnType<typeof setTimeout> | null = null;

export function performSearch(query: string): void {
  setSearchQuery(query);

  if (searchTimer) {
    clearTimeout(searchTimer);
  }

  if (!query.trim()) {
    setSearchResults([]);
    return;
  }

  searchTimer = setTimeout(async () => {
    try {
      const results = await ipcSearch(query);
      setSearchResults(results);
    } catch {
      setSearchResults([]);
    }
  }, 250);
}

export function toggleSearchMode(): void {
  setSearchMode((prev) => !prev);
  if (!searchMode()) {
    setSearchQuery("");
    setSearchResults([]);
  }
}
