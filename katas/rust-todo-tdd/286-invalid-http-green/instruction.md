# Green — Validation is a 422

**File:** `crates/app/src/error.rs` (part 2 of 10)  
**Save to:** `~/code/todo/crates/app/src/error.rs`

## Goal
Add the variant and its status, and refine the `RegisterError` mapping.

## Why
The problems are sentences, not types: the browser shows them as they are. The domain's errors already explain themselves (`Display` from thiserror).

## Help
- Save with `:w! ~/code/todo/crates/app/src/error.rs`.
