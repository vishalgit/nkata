# Green, part 1 — who is calling?

**File:** `crates/app/src/effects.rs` (part 2 of 4)  
**Save to:** `~/code/todo/crates/app/src/effects.rs`

## Goal
Two effects: read the session token from the request's cookies, and resolve it to a user.

## Why
- `leptos_axum::extract()` runs any Axum extractor against the current request; `HeaderMap` gives the headers.
- `let … else` returns early when there is no cookie: no cookie is not an error, it is "nobody".

## Help
- Save with `:w! ~/code/todo/crates/app/src/effects.rs`.
