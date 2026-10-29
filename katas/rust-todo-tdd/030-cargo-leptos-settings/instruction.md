# cargo-leptos settings

**File:** `Cargo.toml` (part 2 of 2)  
**Save to:** `~/code/todo/Cargo.toml`

## Goal
To serve the page you need cargo-leptos, so its settings arrive now: append the WASM release profile and the `[[workspace.metadata.leptos]]` table.

## Why
- `bin-package` is built for the server, `lib-package` compiled to WASM for the browser. cargo-leptos turns on `ssr` and `hydrate` through those crates' own dependencies on `app`, so no features are listed here.
- `site-addr = "0.0.0.0:3000"`: inside a container, `127.0.0.1` would be unreachable from your browser.
- `wasm-release` trades compile time for a small bundle: `opt-level = "z"`, LTO, one codegen unit, `panic = "abort"`.

## Help
- Save with `:w! ~/code/todo/Cargo.toml`.
