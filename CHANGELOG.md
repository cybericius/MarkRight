# Changelog

All notable changes to MarkRight are documented in this file.

## [Unreleased]

## [0.3.2] — 2026-11-02

### Fixed
- MDX: component tags with `{…}` expressions (`<Chart data={[1, 2]} />`, `{...props}`, multi-line props) and `{/* comments */}` are hidden instead of showing as text
- Windows: installing repairs "Open with" entries that still point at an earlier MarkRight install in another folder, and uninstalling removes MarkRight's entries
- Page Down, Page Up, Home, End, Space and the arrow keys scroll the document right after it opens, without clicking into it first

## [0.3.1] — 2026-11-02

### Added
- Settings shows the app version
- MDX documents (`.mdx`) open like Markdown: listed in the file tree, searched, linkable, and registered as a file association (the Linux packages install an MDX MIME type, since the system database classifies `.mdx` as a game ROM). Top-level `import`/`export` lines are hidden; JSX component tags are dropped and the Markdown inside them is kept

### Fixed
- `.markdown`, `.mdown`, `.mkd` and upper-case `.MD` files now appear in the file tree and in search (only lower-case `.md` did); wikilinks to them no longer get `.md` appended

## [0.3.0] — 2026-10-31

### Added
- Mermaid diagrams in ```` ```mermaid ```` code blocks
- Graphviz diagrams in ```` ```dot ```` and ```` ```graphviz ```` code blocks
- Math rendering with KaTeX: `$inline$`, `$$display$$` and ```` ```math ```` blocks
- Footnotes, collected at the end of the document with back-references
- GitHub alerts and Obsidian callouts (`> [!NOTE]`), including Obsidian types, aliases, custom titles and folding
- Emoji shortcodes such as `:smile:`
- Wikilinks: `[[Page]]`, `[[Page#Heading|alias]]`, image embeds `![[image.png|300]]` and note embeds `![[Note]]` / `![[Note#Section]]`
- Relative images load from the document's folder
- Links to local files (PDFs, images, and other documents) open in their default app
- Raw HTML that wraps Markdown (`<details>`, `<kbd>`, aligned `<div>`) renders as a single element

### Security
- Raw HTML and link URLs in documents are sanitized before rendering
- Content Security Policy for the app window

## [0.2.8] — 2026-10-10

### Fixed
- Ctrl+A now selects only the document content instead of the whole window (sidebars, status bar)

## [0.2.7] — 2026-03-14

### Fixed
- White flash on startup when dark mode is active (theme is now applied before first paint)

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
