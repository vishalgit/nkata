# Create the crate

**File:** `Cargo.toml`
**Save to:** `~/code/design-kit/Cargo.toml`

## Goal
Create the `design-kit` crate and type its `Cargo.toml`.

## Run first
```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --locked          # Trunk 0.22: builds wasm + compiles SCSS
mkdir -p ~/code && cd ~/code
cargo new design-kit && cd design-kit
mkdir -p style/{abstracts,vendors,base,layout,components,pages,themes}
mkdir -p src/{components,pages}
```

## Why
- **Leptos 0.8** is the current stable line (0.8.22). 0.9 exists only as a release candidate, so stay on `"0.8"`.
- The `csr` feature means client-side rendering: the whole app is a wasm bundle in the browser. No server, so all your attention goes to the design system.
- Trunk reads `index.html`, builds the wasm, and compiles `style/main.scss` with Dart Sass. You never run `sass` yourself.
- The `style/` folders you just made are the **7** of 7-1. `style/main.scss` is the **1**.
- `opt-level = "z"` and `lto` keep the release wasm small.

## Check
`cargo check` finishes. (It builds for your host; that's fine for checking.)

## Help
- Save each kata into the project with `<leader>kw` and the path from the **File** line, e.g. `~/code/design-kit/Cargo.toml`.
- If Trunk says it can't find `wasm-bindgen`, let it download it; Trunk picks the version that matches your `Cargo.lock`.
