# Green, part 2 — the delete slice

**File:** `crates/app/src/features/todos/delete.rs` (part 1 of 2)  
**Save to:** `~/code/todo/crates/app/src/features/todos/delete.rs`

## Goal
No workflow: deleting has no business rule beyond "may you touch it" — which `access.rs` already answers.

## Why
The second delete is `404` because `load_for` cannot find the row any more.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/delete.rs`.
