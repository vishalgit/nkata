# Green, part 3 — the list slice

**File:** `crates/app/src/features/todos/list.rs` (part 1 of 25)  
**Save to:** `~/code/todo/crates/app/src/features/todos/list.rs`

## Goal
Turn three raw URL parameters into one `TodoQuery`, and let the Chapter 5 SQL do the work.

## Why
- The owner is the session's user: the SQL `WHERE owner_id = $1` is the second half of TODO-16.
- Missing form fields arrive as `None`, so an empty `POST` lists page 1 of everything.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/list.rs`.
