# Green, part 1 — the add form joins the create slice

**File:** `crates/app/src/features/todos/create.rs` (part 2 of 2)  
**Save to:** `~/code/todo/crates/app/src/features/todos/create.rs`

## Goal
Append `CreateTodoForm` to `create.rs`.

## Why
The form takes the action as a prop instead of creating it, so the board can watch the same action and refetch the list.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/create.rs`.
