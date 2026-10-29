# Green, part 1 — the shell and the App component

**File:** `crates/app/src/app.rs` (part 1 of 7)  
**Save to:** `~/code/todo/crates/app/src/app.rs`

## Goal
Type `app.rs`: the HTML shell (server only) and the root component (server and browser).

## Why
- `shell` renders the whole document once, on the server. `<HydrationScripts>` loads the WASM that takes the page over in the browser; `<AutoReload>` is cargo-leptos's live reload in development.
- `App` runs twice: on the server to produce HTML, in the browser to hydrate it.
- `/pkg/todo.css` is the compiled SCSS; the name `todo` comes from the cargo-leptos settings.
- `class="l-main"` is a layout class from the 7-1 SCSS of Chapter 8.

## Help
- Save with `:w! ~/code/todo/crates/app/src/app.rs`.
