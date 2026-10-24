# Trunk's entry page

**File:** `index.html`
**Save to:** `~/code/design-kit/index.html`

## Goal
Type the `index.html` Trunk builds from.

## Why
- `<link data-trunk rel="scss" href="style/main.scss">` is the whole SCSS pipeline. Trunk compiles that one file; every other partial is reached through `@use` from it.
- `<link data-trunk rel="rust">` builds the crate in this folder into wasm and injects the loader script.
- `data-wasm-opt="z"` runs `wasm-opt` for size on `trunk build --release`.
- The body is empty on purpose: Leptos mounts into it.

## Check
Nothing to run yet; `src/main.rs` still prints hello world.

## Help
