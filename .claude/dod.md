# Definition of Done — MarkRight

## Criteria

### Rust tests
- **Applies when**: New `.rs` files added under `crates/` or `src-tauri/` (not test modules themselves)
- **Check**: New public functions have corresponding tests
- **How to verify**: For new public `fn` items, check that `#[cfg(test)]` module exists in the same file or a sibling test file covers the functionality. Skip trivial structs, type aliases, and Tauri command wrappers.

### Clippy clean
- **Applies when**: Any `.rs` files changed
- **Check**: No new clippy warnings introduced
- **How to verify**: `cargo clippy` passes without warnings. Pedantic lints are enabled via `.cargo/config.toml` — do not add `#[allow]` attributes without justification.

### Frontend builds
- **Applies when**: Any `.ts`, `.tsx`, or `.css` files changed
- **Check**: `pnpm build` succeeds without errors
- **How to verify**: TypeScript strict mode passes, no unresolved imports, no build warnings treated as errors.

### Task tracking
- **Applies when**: A feature, bug fix, or enhancement listed in `TO-DOS.md` is implemented
- **Check**: The corresponding item in `TO-DOS.md` is marked done in the same commit
- **How to verify**: If the diff implements a tracked item, `TO-DOS.md` should also appear in the diff with the relevant `- [ ]` changed to `- [x]`.
