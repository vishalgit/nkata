# Green, part 4 — the create slice

**File:** `crates/app/src/features/todos/create.rs` (part 1 of 2)  
**Save to:** `~/code/todo/crates/app/src/features/todos/create.rs`

## Goal
Require a user, decide with the domain workflow, insert, respond.

## Why
- The `#[server]` macro turns `create_todo(title)` into a struct named `CreateTodo`; two things with one name in one scope will not compile, hence `as Command` and `as decide`.
- The owner comes from the session, never from the form: a user cannot create a todo for someone else.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/create.rs`.
