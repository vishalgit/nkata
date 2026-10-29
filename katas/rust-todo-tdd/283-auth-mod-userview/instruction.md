# Piece 3 — the UserView DTO

**File:** `crates/app/src/features/auth/mod.rs` (part 4 of 8)  
**Save to:** `~/code/todo/crates/app/src/features/auth/mod.rs`

## Goal
Declare the `register` slice and the DTO the browser receives.

## Why
`register` is not behind `ssr`: the slice compiles in both builds. Its `#[server]` function becomes a real function on the server and an HTTP call in the browser.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/mod.rs`.
