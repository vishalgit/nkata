# Green — command in, event out

**File:** `crates/domain/src/todos/workflows/create.rs` (part 2 of 4)  
**Save to:** `~/code/todo/crates/domain/src/todos/workflows/create.rs`

## Goal
Type the command, the event, an empty error type, and the workflow.

## Why
- The **command** holds raw input — `title` is a `String`, not a `TodoTitle` — because validating it is the workflow's job.
- The **event** is a fact in the past tense, holding the new `ActiveTodo`.
- The error type exists but is empty; the next test fills it.
- `id` and `now` are dependencies passed in as plain values: the test chooses them, so the result is exact.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/workflows/create.rs`.
