# Green, part 1 — TodoPage

**File:** `crates/app/src/features/todos/view.rs` (part 2 of 2)  
**Save to:** `~/code/todo/crates/app/src/features/todos/view.rs`

## Goal
The list answers with a **page** from its first test, so its contract never changes later.

## Why
`Paged<Todo>` stays on the server; `TodoPage` is its browser-shaped copy: views instead of domain todos, plain numbers instead of `PageRequest`.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/view.rs`.
