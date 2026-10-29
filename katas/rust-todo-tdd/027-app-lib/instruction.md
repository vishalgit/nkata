# Green, part 1 — the app crate's lib.rs

**File:** `crates/app/src/lib.rs` (part 1 of 4)  
**Save to:** `~/code/todo/crates/app/src/lib.rs`

## Goal
Declare the two modules; `state` exists only with `ssr`.

## Why
`#[cfg(feature = "ssr")]` keeps `AppState` (and with it `sqlx` and `axum`) out of the browser build.

## Help
- Save with `:w! ~/code/todo/crates/app/src/lib.rs`.
