# Green — escape_like

**File:** `crates/app/src/features/todos/store.rs` (part 12 of 21)  
**Save to:** `~/code/todo/crates/app/src/features/todos/store.rs`

## Goal
Escape the backslash first, then `%` and `_`.

## Why
Backslash first, so the backslashes added for `%` and `_` are not doubled again. Backslash is Postgres's default `LIKE` escape character.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/store.rs`.
