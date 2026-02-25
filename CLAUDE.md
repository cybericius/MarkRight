# MarkRight

Native, lightweight, directory-first Markdown viewer built with Tauri v2 + SolidJS.

## Stack
- **Backend**: Rust (Tauri v2), comrak for Markdown parsing
- **Frontend**: SolidJS + Vite + Tailwind CSS v4
- **Workspace**: Cargo workspace at root with `src-tauri`, `crates/markright-core`, `crates/markright-syntax`

## Commands
- `pnpm dev` — Start Vite dev server (frontend only)
- `pnpm tauri dev` — Full Tauri dev mode (Rust + frontend)
- `pnpm build` — Build frontend for production
- `cargo build` — Build Rust workspace
- `cargo test` — Run all Rust tests
- `cargo clippy` — Run lints (pedantic enabled via `.cargo/config.toml`)

## Conventions
- Rust edition 2024, clippy pedantic
- Frontend: TypeScript strict, SolidJS components in `src/components/`
- License: AGPL-3.0-only
- Track tasks in `TO-DOS.md` — mark items done in the same commit as implementation

## Architecture
- `crates/markright-core/` — Parser, tree builder, TOC, frontmatter, config (shared library)
- `crates/markright-syntax/` — Syntax highlighting (stub, uses syntect)
- `src-tauri/` — Tauri app, IPC commands, app state
- `src/` — SolidJS frontend with three-panel layout
