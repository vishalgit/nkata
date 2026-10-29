# Green, part 2 — the frontend crate

**File:** `crates/frontend/src/lib.rs`  
**Save to:** `~/code/todo/crates/frontend/src/lib.rs`

## Goal
Add the frontend's dependencies, make it a `cdylib`, and type its one function.

## Run first
```sh
$ bin/dev cargo add -p frontend app --path crates/app --features hydrate
$ bin/dev cargo add -p frontend leptos@0.8 --features hydrate
$ bin/dev cargo add -p frontend wasm-bindgen console_error_panic_hook
```

In `crates/frontend/Cargo.toml`, add:

```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

## Why
- `cdylib` is what wasm-bindgen turns into `/pkg/todo.wasm`; `rlib` keeps it usable as a normal Rust library.
- `hydrate()` is exported to JavaScript. The loader calls it once the WASM arrives, and Leptos attaches to the server-rendered `<body>` instead of re-rendering it.
- `console_error_panic_hook` turns a Rust panic in the browser into a readable console error.

## Help
- Save with `:w! ~/code/todo/crates/frontend/src/lib.rs`.
