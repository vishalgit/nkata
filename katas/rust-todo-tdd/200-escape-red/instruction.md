# Test 6 (pure): wildcards in user input are literal

**File:** `crates/app/src/features/todos/store.rs` (part 11 of 21)  
**Save to:** `~/code/todo/crates/app/src/features/todos/store.rs`

## Goal
In SQL `LIKE`, `%` and `_` are wildcards; a search for `50%` must not match everything. Append a plain unit test for an escaper.

## Why
A pure function in the store module: a `#[test]`, not a `#[sqlx::test]`. It needs no database.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/store.rs`.
