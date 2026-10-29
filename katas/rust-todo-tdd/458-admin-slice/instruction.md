# Green, part 3 — the all-todos slice

**File:** `crates/app/src/features/admin/all_todos.rs` (part 1 of 4)  
**Save to:** `~/code/todo/crates/app/src/features/admin/all_todos.rs`

## Goal
The admin query joins `todos` with `users` and serves one slice, so its SQL lives **in the slice**.

## Why
- `authorize(…)?` before touching the database: the policy decides first.
- `AS "completed!"` tells sqlx the expression is never `NULL`.
- SQL used by one slice stays in that slice; nothing else needs owners' emails.

## Check
`bin/dev cargo t-http` — green. Refresh `.sqlx/`.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/admin/all_todos.rs`.
