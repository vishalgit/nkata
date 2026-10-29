# Green — rename

**File:** `crates/domain/src/todos/workflows/rename.rs` (part 2 of 5)  
**Save to:** `~/code/todo/crates/domain/src/todos/workflows/rename.rs`

## Goal
Parse the new title, remember the old one, retitle.

## Why
- The event holds a `Todo`, not an `ActiveTodo`: renaming works in any state.
- The old title is in the event because "what changed" is part of the fact — an audit log or an undo would need it.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/workflows/rename.rs`.
