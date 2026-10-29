# Green — limit and offset

**File:** `crates/domain/src/todos/query.rs` (part 4 of 14)  
**Save to:** `~/code/todo/crates/domain/src/todos/query.rs`

## Goal
Add `limit` and `offset`.

## Why
`u64`: these become SQL `LIMIT` and `OFFSET`. `u64::from` is a lossless widening, so no `as` cast is needed.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/query.rs`.
