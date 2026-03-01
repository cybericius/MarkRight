# Changelog

All notable changes to MarkRight are documented in this file.

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
