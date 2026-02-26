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

## Model Selection

Use the cheapest model that can handle the task. Switch with `/model <name>`.

| Model | Use for |
|---|---|
| **haiku** | Test runs, shell commands, file search/grep, simple edits, git ops, Docker, infra |
| **sonnet** | Regular coding, features, bug fixes, code review, refactoring, docs |
| **opus** | Complex planning, architecture, multi-file design, difficult debugging, security review |

### Subagent Rules (MANDATORY)

- **Always** set `model: "haiku"` for Explore, Bash, and simple subagents
- **Only** use sonnet/opus for subagents doing complex reasoning
- Slash commands (`/flush`, `/commit`, `/ship`) use haiku-level operations

### When NOT to Use Opus

- Simple file searches or greps
- Running tests or build commands
- Single-file edits with clear requirements
- Git operations, Docker commands
- Reading/summarizing files

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
