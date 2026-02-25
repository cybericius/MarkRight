import { openFolder as ipcOpenFolder, getDocument, search as ipcSearch, getConfig, saveConfig, checkLicense, activateLicense as ipcActivateLicense, getInitialFile } from "../lib/tauri";
import type { AppConfig } from "../lib/types";
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
  setLicenseStatus,
  setShowUpgradePrompt,
  setUpgradeFeatureName,
  setLeftPanelWidth,
  setRightPanelWidth,
  setShowLeftPanel,
  setShowRightPanel,
  setFontFamilyUi,
  setFontSizeUi,
  setFontFamilyContent,
  setFontSizeContent,
  setLineHeightContent,
  setTheme,
  setZoom,
  setContentWidth,
  zoom,
  contentWidth,
  leftPanelWidth,
  rightPanelWidth,
  showLeftPanel,
  showRightPanel,
  fontFamilyUi,
  fontSizeUi,
  fontFamilyContent,
  fontSizeContent,
  lineHeightContent,
  theme,
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

let darkMediaQuery: MediaQueryList | null = null;
let darkMediaHandler: ((e: MediaQueryListEvent) => void) | null = null;

function applyDarkMode(): void {
  const root = document.documentElement;
  const t = theme();

  // Clean up previous system listener
  if (darkMediaQuery && darkMediaHandler) {
    darkMediaQuery.removeEventListener("change", darkMediaHandler);
    darkMediaHandler = null;
  }

  if (t === "dark") {
    root.classList.add("dark");
  } else if (t === "light") {
    root.classList.remove("dark");
  } else {
    // system
    darkMediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    const apply = (matches: boolean) => {
      if (matches) root.classList.add("dark");
      else root.classList.remove("dark");
    };
    apply(darkMediaQuery.matches);
    darkMediaHandler = (e) => apply(e.matches);
    darkMediaQuery.addEventListener("change", darkMediaHandler);
  }
}

function applyCssVars(): void {
  const root = document.documentElement;
  root.style.setProperty("--font-ui", fontFamilyUi());
  root.style.setProperty("--font-ui-size", `${fontSizeUi()}px`);
  root.style.setProperty("--font-content", fontFamilyContent());
  root.style.setProperty("--font-content-size", `${fontSizeContent()}px`);
  root.style.setProperty("--font-content-line-height", `${lineHeightContent()}`);
  root.style.setProperty("--content-zoom", `${zoom() / 100}`);
  applyDarkMode();
}

export async function loadConfig(): Promise<void> {
  try {
    const cfg = await getConfig();
    setLeftPanelWidth(cfg.left_panel_width);
    setRightPanelWidth(cfg.right_panel_width);
    setShowLeftPanel(cfg.show_left_panel);
    setShowRightPanel(cfg.show_right_panel);
    setFontFamilyUi(cfg.font_family_ui);
    setFontSizeUi(cfg.font_size_ui);
    setFontFamilyContent(cfg.font_family_content);
    setFontSizeContent(cfg.font_size_content);
    setLineHeightContent(cfg.line_height_content);
    setTheme(cfg.theme);
    setZoom(cfg.zoom);
    setContentWidth(cfg.content_width);
    applyCssVars();
  } catch {
    // Use defaults — CSS vars already set in global.css
  }
}

function currentConfig(): AppConfig {
  return {
    theme: theme(),
    left_panel_width: leftPanelWidth(),
    right_panel_width: rightPanelWidth(),
    show_left_panel: showLeftPanel(),
    show_right_panel: showRightPanel(),
    font_family_ui: fontFamilyUi(),
    font_size_ui: fontSizeUi(),
    font_family_content: fontFamilyContent(),
    font_size_content: fontSizeContent(),
    line_height_content: lineHeightContent(),
    zoom: zoom(),
    content_width: contentWidth(),
  };
}

let saveTimer: ReturnType<typeof setTimeout> | null = null;

export function persistConfig(): void {
  if (saveTimer) clearTimeout(saveTimer);
  saveTimer = setTimeout(() => {
    saveConfig(currentConfig()).catch(() => {});
  }, 500);
}

export async function loadLicense(): Promise<void> {
  try {
    const status = await checkLicense();
    setLicenseStatus(status);
  } catch {
    // No license or error — stay on free tier
  }
}

export async function activateLicense(key: string): Promise<boolean> {
  try {
    const status = await ipcActivateLicense(key);
    setLicenseStatus(status);
    return status.valid;
  } catch {
    return false;
  }
}

export function promptUpgrade(featureName: string): void {
  setUpgradeFeatureName(featureName);
  setShowUpgradePrompt(true);
}

export async function handleInitialFile(): Promise<void> {
  try {
    const initial = await getInitialFile();
    if (initial) {
      await openFolder(initial.folder_path);
      await openDocument(initial.file_path);
    }
  } catch {
    // No initial file or error — ignore
  }
}

export function updateConfig(partial: Partial<AppConfig>): void {
  if (partial.theme !== undefined) setTheme(partial.theme);
  if (partial.left_panel_width !== undefined) setLeftPanelWidth(partial.left_panel_width);
  if (partial.right_panel_width !== undefined) setRightPanelWidth(partial.right_panel_width);
  if (partial.show_left_panel !== undefined) setShowLeftPanel(partial.show_left_panel);
  if (partial.show_right_panel !== undefined) setShowRightPanel(partial.show_right_panel);
  if (partial.font_family_ui !== undefined) setFontFamilyUi(partial.font_family_ui);
  if (partial.font_size_ui !== undefined) setFontSizeUi(partial.font_size_ui);
  if (partial.font_family_content !== undefined) setFontFamilyContent(partial.font_family_content);
  if (partial.font_size_content !== undefined) setFontSizeContent(partial.font_size_content);
  if (partial.line_height_content !== undefined) setLineHeightContent(partial.line_height_content);
  if (partial.zoom !== undefined) setZoom(Math.max(25, Math.min(300, partial.zoom)));
  if (partial.content_width !== undefined) setContentWidth(partial.content_width);
  applyCssVars();
  persistConfig();
}
