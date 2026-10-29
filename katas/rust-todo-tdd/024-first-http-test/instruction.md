# Red — the first HTTP test

**File:** `crates/server/tests/skeleton.rs` (part 1 of 4)  
**Save to:** `~/code/todo/crates/server/tests/skeleton.rs`

## Goal
Write the first test: `GET /` answers `200` with an `<h1>Todo</h1>`.

## Check
```sh
$ bin/dev cargo t-http
```
**Red**: a page of unresolved crates (`tokio`, `axum`, `app`, `tower`, …). The compiler is now your dependency list; the next katas add each one.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/skeleton.rs`.
