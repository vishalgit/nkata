# Green — AdminTodoTable

**File:** `crates/app/src/features/admin/all_todos.rs` (part 3 of 4)  
**Save to:** `~/code/todo/crates/app/src/features/admin/all_todos.rs`

## Goal
A table that reuses the delete slice's action from context.

## Why
A UI may call another slice's **endpoint**; slices never call each other's **internals**. The delete action posts to `/api/todos/delete`, where the policy lets admins through.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/admin/all_todos.rs`.
