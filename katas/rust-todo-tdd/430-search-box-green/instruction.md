# Green — a plain GET form

**File:** `crates/app/src/features/todos/list.rs` (part 25 of 25)  
**Save to:** `~/code/todo/crates/app/src/features/todos/list.rs`

## Goal
A form that navigates to `/?q=…&filter=…`.

## Why
A plain `GET` form needs no JavaScript: the browser navigates, the server renders the result. The hidden `filter` keeps the current tab.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/list.rs`.
