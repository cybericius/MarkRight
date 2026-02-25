# MarkRight

Native, lightweight, directory-first Markdown viewer.

<!-- TODO: Add screenshot -->

## Features

- **Directory browsing** — open any folder and navigate its Markdown files in a collapsible file tree
- **GFM rendering** — full GitHub Flavored Markdown support (tables, task lists, strikethrough, autolinks)
- **Syntax highlighting** — code blocks with language-aware highlighting via syntect
- **Table of contents** — auto-generated from headings with click-to-scroll and scroll tracking
- **Cross-file search** — grep-like search across all Markdown files in a directory *(Pro)*
- **In-file search** — Ctrl+F find with match highlighting, count, and prev/next navigation
- **Resizable panels** — three-panel layout with draggable dividers and collapsible sidebars
- **Font & theme settings** — configurable content/UI fonts, sizes, line height, light/dark/system theme
- **Frontmatter support** — YAML frontmatter parsing and display
- **Persistent config** — settings saved to disk and restored on launch

## Build from Source

### Prerequisites

- [Rust](https://rustup.rs/) 1.75+
- [Node.js](https://nodejs.org/) 20+
- [pnpm](https://pnpm.io/) 10+
- Platform dependencies for [Tauri v2](https://v2.tauri.app/start/prerequisites/)

### Build

```sh
git clone https://github.com/cybericius/MarkRight.git
cd MarkRight
pnpm install
pnpm tauri build
```

The built application will be in `src-tauri/target/release/`.

## Development

```sh
pnpm tauri dev      # Full Tauri dev mode (Rust + frontend)
cargo test          # Run all Rust tests
cargo clippy        # Run lints (pedantic enabled)
pnpm build          # Build frontend only
```

## License

MarkRight is licensed under [AGPL-3.0-only](LICENSE).

Pro features (cross-file search and future extensions) are unlocked via a donation-based license key. Support the project and get your key at [polar.sh/cybericius](https://polar.sh/cybericius).
