import { invoke } from "@tauri-apps/api/core";
import type { DocumentResponse, TreeNode, TocEntry, SearchResult } from "./types";

export async function openFolder(path: string): Promise<TreeNode[]> {
  return invoke<TreeNode[]>("open_folder", { path });
}

export async function getTree(): Promise<TreeNode[]> {
  return invoke<TreeNode[]>("get_tree");
}

export async function getDocument(path: string): Promise<DocumentResponse> {
  return invoke<DocumentResponse>("get_document", { path });
}

export async function getToc(path: string): Promise<TocEntry[]> {
  return invoke<TocEntry[]>("get_toc", { path });
}

export async function search(query: string): Promise<SearchResult[]> {
  return invoke<SearchResult[]>("search", { query });
}
