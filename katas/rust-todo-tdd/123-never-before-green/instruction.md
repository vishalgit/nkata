# Green — clamp to the creation time

**File:** `crates/domain/src/todos/todo.rs` (part 6 of 10)  
**Save to:** `~/code/todo/crates/domain/src/todos/todo.rs`

## Goal
Record the later of the two times.

## Why
- This invariant is why `CompletedTodo`'s fields are private: the only way to make one is `complete`, and `complete` enforces it.
- The fields are listed `completed_at` first because `self.0.created_at` must be read before `self.0` moves into `info`.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/todo.rs`.
