# Green — a second state and a transition

**File:** `crates/domain/src/todos/todo.rs` (part 4 of 10)  
**Save to:** `~/code/todo/crates/domain/src/todos/todo.rs`

## Goal
Add `CompletedTodo`, the `complete` transition, and the `Completed` variant.

## Why
- `complete(self, …)` takes the active todo **by value**: after the call it is gone.
- The compiler pointed at `info()` the moment the variant appeared — a `match` must cover every state.
- `matches!` is a `match` that returns `bool`.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/todo.rs`.
