# Changelog

All notable changes to MarkRight are documented in this file.

## [Unreleased]

## [0.2.6] — 2026-03-11

### Fixed
- Code blocks had white background on first load when system theme is dark (race condition in isDark signal)
- Code block font size now scales proportionally with content font (was fixed at 14px)
- Default body font changed from serif to IBM Plex Sans for cleaner reading

## [0.2.5] — 2026-03-02

### Added
- Selectable code syntax themes — choose between Ocean (default) and Sulphurpool in Settings > View (Pro)
- Sulphurpool dark & light themes (Atelier base16 palette by Bram de Haan)
- Proper light-mode syntax highlighting using dedicated light themes (replaces CSS color stripping)
- `code_theme` persisted in user config

### Fixed
- Light theme contrast — bumped too-faint grays (`text-gray-400` → `text-gray-500`) across 7 components
- StatusBar separator nearly invisible in light mode (`text-gray-300` → `text-gray-400`)
- Settings section headers too light in light mode
- Code blocks had no syntax coloring in light mode (dark theme colors were stripped by CSS hack)

## [0.2.4] — 2026-03-01

### Fixed
- F11 fullscreen not working — added missing Tauri v2 window permissions

## [0.2.3] — 2026-03-01

### Fixed
- App not appearing in "Open With" file association dialogs (missing `%f` in .desktop Exec line)
- Dynamic version in StatusBar — now auto-reads from package.json at build time
- Added `Categories` and `text/x-markdown` MIME type to .desktop file

## [0.2.2] — 2026-03-01

### Added
- Floating app icon in top-left corner during fullscreen mode
- App icons served from frontend public directory

### Changed
- F11 fullscreen toggle now tracks state via signal for UI reactivity

## [0.2.1] — 2026-02-26

### Added
- F11 fullscreen toggle
- File associations for .md, .markdown, .mdown, .mkd

### Fixed
- Font controls and light mode code block styling
- Default content font bumped to 18px
- App icons reverted to no-padding with white background
- All icon PNGs ensured RGBA (required by Tauri)
- TOC hover artifacts

## [0.2.0] — 2026-02-26

### Added
- Zoom controls (Ctrl+=/Ctrl+-/Ctrl+0, 25–300%) with StatusBar buttons and Settings slider
- Content width modes: Default (max-w-3xl), Fit, A4 (210mm)
- Dark mode with system preference detection and manual toggle
- CI workflow: tests on push, cross-platform builds on tag (Linux, macOS, Windows)
- CLI argument support — open a file directly from the command line
- Custom MarkRight branding icons
- License activation, settings persistence, and production keys
- Zoom and content width persisted in config

### Changed
- Overhauled README with feature descriptions, Pro section, and license model

## [0.1.0-alpha] — 2026-02-25

### Added
- Initial project bootstrap: Tauri v2 + SolidJS + Cargo workspace
- Core Markdown viewer with comrak parser and AST serialization
- Syntax highlighting via syntect
- Three-panel layout: file tree, content area, TOC
- Folder opening with directory tree navigation
- In-file search with CSS Custom Highlight API (Ctrl+F)
- Cross-file grep search (Ctrl+Shift+F) — Pro feature
- Settings panel with font, panel, and theme controls
- License key system with feature gating
