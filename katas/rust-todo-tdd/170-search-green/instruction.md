# Green — the minimum

**File:** `crates/domain/src/todos/query.rs` (part 8 of 14)  
**Save to:** `~/code/todo/crates/domain/src/todos/query.rs`

## Goal
A newtype, an empty error type, and a `parse` that always says "no search".

## Why
Three states fit in the return type: `Ok(None)` (no search), `Ok(Some(term))` (search), `Err(_)` (refused). The minimum that passes uses only the first.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/query.rs`.
