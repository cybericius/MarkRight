# MarkRight — Roadmap & Tasks

## Phase 0: Project Bootstrap
- [x] Initialize Tauri v2 + SolidJS + Vite project
- [x] Set up Cargo workspace with `markright-core` crate
- [ ] CI: GitHub Actions build on Linux/macOS/Windows + Rust tests
- [x] AGPL-3.0 license, project scaffolding
- [ ] Coding conventions (clippy strict, ESLint+Prettier)

## Phase 1a: Core Viewer
- [ ] **Rust**: comrak integration, AST→JSON serialization
- [ ] **Rust**: Recursive `.md` folder scanning, tree structure building
- [ ] **Rust**: TOC extraction from AST (heading hierarchy + IDs)
- [ ] **Tauri commands**: `open_folder`, `get_document`, `get_tree`, `get_toc`
- [ ] **UI**: Three-panel layout with resizable, collapsible panels
- [ ] **UI**: File tree with collapsible directories, active file highlight
- [ ] **UI**: Markdown rendering from AST (all GFM elements)
- [ ] **UI**: TOC panel with click-to-scroll
- [ ] **UI**: Scroll tracking (intersection observer → active TOC heading)
- [ ] **UI**: Internal link navigation within app
- [ ] **UI**: Syntax-highlighted code blocks

## Phase 1b: Search & Polish
- [ ] **Rust**: tantivy full-text index, incremental updates
- [ ] **Rust**: File watcher (notify), auto-reload on changes
- [ ] **UI**: Search panel with results, snippets, navigate to match
- [ ] **UI**: Light/dark theme with system detection
- [ ] **UI**: Command palette (Ctrl+P) for quick document switch
- [ ] **UI**: Keyboard shortcuts for panel toggling, tree navigation
- [ ] **UI**: Responsive layout (panels collapse on narrow windows)
- [ ] **UI**: `SUMMARY.md` support for custom tree ordering

## Phase 1c: Release v1.0
- [ ] Platform testing (Windows, macOS, Linux)
- [ ] Signed builds + auto-update (Tauri updater plugin)
- [ ] App icon and branding
- [ ] User documentation (dogfooded in MarkRight itself)
- [ ] Release: GitHub, Homebrew, Scoop/Winget, AUR
