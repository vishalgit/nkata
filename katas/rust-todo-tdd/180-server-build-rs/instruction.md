# Rebuild when migrations change — build.rs

**File:** `crates/server/build.rs`  
**Save to:** `~/code/todo/crates/server/build.rs`

## Goal
So cargo notices a new migration, type a build script that watches the folder.

## Why
`migrate!` reads the folder at compile time, but cargo only rebuilds when Rust sources change. Without this, a new migration would not reach the binary until something else changed.

## Help
- Save with `:w! ~/code/todo/crates/server/build.rs`.
