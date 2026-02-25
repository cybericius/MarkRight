import { createSignal, createMemo } from "solid-js";
import type {
  TreeNode,
  MdNode,
  TocEntry,
  Frontmatter,
  SearchResult,
  LicenseStatus,
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

// Font / config
export const [fontFamilyUi, setFontFamilyUi] = createSignal("system-ui");
export const [fontSizeUi, setFontSizeUi] = createSignal(13);
export const [fontFamilyContent, setFontFamilyContent] = createSignal("serif");
export const [fontSizeContent, setFontSizeContent] = createSignal(16);
export const [lineHeightContent, setLineHeightContent] = createSignal(1.6);
export const [theme, setTheme] = createSignal<"light" | "dark" | "system">("system");

// License
export const [licenseStatus, setLicenseStatus] = createSignal<LicenseStatus>({
  valid: false,
  email: null,
  tier: null,
});
export const isLicensed = createMemo(() => licenseStatus().valid);
export const [showUpgradePrompt, setShowUpgradePrompt] = createSignal(false);
export const [upgradeFeatureName, setUpgradeFeatureName] = createSignal("");

// In-file search (FindBar)
export const [findBarOpen, setFindBarOpen] = createSignal(false);
export const [findQuery, setFindQuery] = createSignal("");
export const [findMatchCount, setFindMatchCount] = createSignal(0);
export const [findCurrentIndex, setFindCurrentIndex] = createSignal(0);

// Settings panel visibility
export const [showSettings, setShowSettings] = createSignal(false);
