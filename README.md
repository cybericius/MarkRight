# MarkRight

Native, lightweight, directory-first Markdown viewer built with Tauri + SolidJS.

## Download

Grab the latest release for your platform from [GitHub Releases](https://github.com/cybericius/MarkRight/releases).

| Platform | Format |
|----------|--------|
| Linux | `.deb`, `.AppImage` |
| macOS (Apple Silicon) | `.dmg` |
| macOS (Intel) | `.dmg` |
| Windows | `.msi`, `.exe` |

## Features

### Reading
- **Open any Markdown file** — double-click or "Open With" from your file manager
- **GFM rendering** — full GitHub Flavored Markdown support (tables, task lists, strikethrough, autolinks)
- **Syntax highlighting** — code blocks with language-aware highlighting via syntect
- **Table of contents** — auto-generated from headings with click-to-scroll and scroll tracking
- **Frontmatter support** — YAML frontmatter parsing and display
- **In-file search** — Ctrl+F find with match highlighting, count, and prev/next navigation

### Pro
- **Directory browsing** — open any folder and navigate its Markdown files in a collapsible file tree *(Pro)*
- **Cross-file search** — grep-like search across all Markdown files in a directory *(Pro)*
- **Internal link navigation** — click links between Markdown files without leaving the app *(Pro)*

### Appearance
- **Light & dark themes** — manual toggle or follow system preference
- **Zoom controls** — Ctrl+=/Ctrl+-/Ctrl+0, 25–300% range
- **Content width modes** — Default (readable width), Fit (full width), or A4 (print-ready)
- **Font & typography settings** — configurable content/UI fonts, sizes, and line height
- **Resizable panels** — three-panel layout with draggable dividers and collapsible sidebars

### Settings
- **Persistent config** — all settings saved to disk and restored on launch
- **Settings panel** — accessible via gear icon or Ctrl+,

## Pro Features

Directory browsing, cross-file search, internal link navigation, and future premium extensions are unlocked with a Pro license key. The free version opens individual Markdown files with full rendering, search, and customization.

Purchase a license to support development and unlock Pro features:

[Get MarkRight Pro](https://buy.polar.sh/polar_cl_US7q3B0057nkiNtmToO7TegiQc6b6EMUeDD5k1ip2kD)

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

## Support the Project

MarkRight is built and maintained by a solo developer. If you find it useful, consider supporting continued development:

- [Purchase a Pro license](https://buy.polar.sh/polar_cl_US7q3B0057nkiNtmToO7TegiQc6b6EMUeDD5k1ip2kD) — unlocks Pro features and funds development
- Star the repo and spread the word

## License

MarkRight source code is licensed under [AGPL-3.0-only](LICENSE).

**Personal use** is free. **Commercial or company use** (deploying internally, bundling in a product, using within an organization) requires a [Pro license purchase](https://buy.polar.sh/polar_cl_US7q3B0057nkiNtmToO7TegiQc6b6EMUeDD5k1ip2kD) which grants usage rights without AGPL obligations.
