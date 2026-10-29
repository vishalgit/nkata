# Green — only the owner

**File:** `crates/app/src/features/todos/access.rs` (part 2 of 3)  
**Save to:** `~/code/todo/crates/app/src/features/todos/access.rs`

## Goal
Compare the todo's owner with the actor.

## Why
- Why `404` and not `403`? A `403` confirms the id exists; `404` leaks nothing.
- Every later slice that uses `load_for` is protected for free.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/access.rs`.
