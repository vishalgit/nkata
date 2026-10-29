# Green — TodoQuery

**File:** `crates/domain/src/todos/query.rs` (part 14 of 14)  
**Save to:** `~/code/todo/crates/domain/src/todos/query.rs`

## Goal
Add `use super::Filter;` and the combined query type.

## Why
Three raw URL strings in, one valid query out. Only the search can fail, so the error is `SearchError`.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/query.rs`.
