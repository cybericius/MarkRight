# BUG: Light theme has too-bright fonts with no contrast against white background

**Reported**: 2026-03-02
**Reporter**: akos
**Status**: FIXED

## Symptom

In light mode, UI text and code blocks had poor contrast against the white background:
- Secondary text (`text-gray-400`) was barely readable
- StatusBar separator (`text-gray-300`) was nearly invisible
- Settings section headers used `text-gray-400` — too light for headings
- Code blocks had no syntax highlighting (all colors stripped by CSS override)

Screenshot: `.bugs/Screenshot_20260302_084115.png`

## Root Cause

Two separate issues:

### 1. Systematic use of too-light Tailwind gray shades

Many components used `text-gray-400` (#9ca3af) and `text-gray-300` (#d1d5db) for text that needed to be readable on white backgrounds. WCAG AA requires 4.5:1 contrast ratio; gray-400 on white is only ~2.7:1.

### 2. Code blocks had no light-mode syntax highlighting

The syntax highlighter (syntect) only generated dark-theme HTML (`base16-ocean.dark`). A CSS hack in `global.css` stripped all inline colors in light mode and replaced them with flat `#1f2937`, eliminating all syntax coloring.

## Fix

### Sub-bug 1: Bump gray shades across 7 components

All `text-gray-400` for readable text bumped to `text-gray-500` (or `text-gray-500 dark:text-gray-400` where dark mode needed preserving):

- **StatusBar.tsx**: separator `text-gray-300` → `text-gray-400`
- **SettingsPanel.tsx**: section headers `text-gray-400` → `text-gray-500 dark:text-gray-400`, close button → `text-gray-500`
- **SearchPanel.tsx**: search icon, clear button, empty states, line numbers all bumped
- **FindBar.tsx**: search icon bumped
- **TreeItem.tsx**: chevron/bullet icons bumped
- **FileTree.tsx**: empty state text bumped
- **TocPanel.tsx**: empty state text bumped

### Sub-bug 2: Add light-mode syntax highlighting

- **markright-syntax/src/lib.rs**: Added `highlight_light()` using `InspiredGitHub` theme, extracted shared `highlight_with_theme()` helper
- **markright-core/src/ast.rs**: Added `highlighted_html_light` field to `MdNode`, populated alongside dark variant for code blocks
- **src/lib/types.ts**: Added `highlighted_html_light` to TypeScript `MdNode` interface
- **src/stores/app.ts**: Added `isDark` derived signal that tracks `<html>` dark class state
- **src/components/markdown/CodeBlock.tsx**: Selects light or dark highlighted HTML based on `isDark()` signal
- **src/styles/global.css**: Removed CSS hack that stripped all syntax colors in light mode

## Verification

- 44 Rust tests pass (`cargo test -p markright-core -p markright-syntax`)
- Frontend builds clean (`pnpm build`)
- Visual verification needed in `pnpm tauri dev` with light theme
