# Green — PageRequest

**File:** `crates/domain/src/todos/query.rs` (part 2 of 14)  
**Save to:** `~/code/todo/crates/domain/src/todos/query.rs`

## Goal
Like `Filter::parse`, a bad URL falls back to something sensible.

## Why
The private field is the invariant: a `PageRequest` with number 0 cannot be built, so `number - 1` in the next kata can never underflow.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/query.rs`.
