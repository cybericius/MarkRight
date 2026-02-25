# MarkRight — Roadmap & Tasks

## Phase 0: Project Bootstrap
- [x] Initialize Tauri v2 + SolidJS + Vite project
- [x] Set up Cargo workspace with `markright-core` crate
- [ ] CI: GitHub Actions build on Linux/macOS/Windows + Rust tests
- [x] AGPL-3.0 license, project scaffolding
- [ ] Coding conventions (clippy strict, ESLint+Prettier)

## Phase 1a: Core Viewer
- [x] **Rust**: comrak integration, AST→JSON serialization
- [x] **Rust**: Recursive `.md` folder scanning, tree structure building
- [x] **Rust**: TOC extraction from AST (heading hierarchy + IDs)
- [x] **Tauri commands**: `open_folder`, `get_document`, `get_tree`, `get_toc`
- [x] **UI**: Three-panel layout with resizable, collapsible panels
- [x] **UI**: File tree with collapsible directories, active file highlight
- [x] **UI**: Markdown rendering from AST (all GFM elements)
- [x] **UI**: TOC panel with click-to-scroll
- [x] **UI**: Scroll tracking (intersection observer → active TOC heading)
- [x] **UI**: Internal link navigation within app
- [x] **UI**: Syntax-highlighted code blocks

## Phase 1b: Search & Polish
- [x] **Rust**: Simple grep-like text search across `.md` files
- [ ] **Rust**: File watcher (notify), auto-reload on changes
- [x] **UI**: Search panel with results, snippets, navigate to match
- [ ] **UI**: Light/dark theme with system detection
- [ ] **UI**: Command palette (Ctrl+P) for quick document switch
- [ ] **UI**: Keyboard shortcuts for panel toggling, tree navigation
- [ ] **UI**: Responsive layout (panels collapse on narrow windows)
- [ ] **UI**: `SUMMARY.md` support for custom tree ordering
- [x] **Settings**: Persistent config (`~/.config/markright/settings.json`)
- [x] **Settings**: Content font family, size, line height controls
- [x] **Settings**: UI font family, size controls
- [x] **Settings**: Panel width/visibility persistence
- [x] **Settings**: Settings modal (gear icon + Ctrl+,)

## Phase 1b.1: In-File Search & Licensing
- [x] **Rust**: License module (Ed25519 token verification, `license.rs`)
- [x] **Tauri**: `check_license` command
- [x] **UI**: In-file search (Ctrl+F) with CSS Custom Highlight API
- [x] **UI**: FindBar with match count, prev/next, keyboard navigation
- [x] **UI**: License status display in StatusBar
- [x] **UI**: Feature gating — cross-file search requires Pro license
- [x] **UI**: UpgradePrompt modal for locked features
- [x] **UI**: ProBadge component for locked feature indicators

## Phase 1c: Release v1.0
- [ ] Platform testing (Windows, macOS, Linux)
- [ ] Signed builds + auto-update (Tauri updater plugin)
- [ ] App icon and branding
- [ ] User documentation (dogfooded in MarkRight itself)
- [ ] Release: GitHub, Homebrew, Scoop/Winget, AUR
