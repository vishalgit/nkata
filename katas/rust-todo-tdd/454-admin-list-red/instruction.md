# HTTP test 2: only admins see all todos

**File:** `crates/server/tests/admin.rs` (part 3 of 4)  
**Save to:** `~/code/todo/crates/server/tests/admin.rs`

**▶ Task**
```sh
TASK_ONLY=1 scripts/backlog task TODO-18 10 ui "Data table (10.8)"
scripts/backlog task TODO-19 10 http "Admin endpoint, table and page (10.6–10.8)" admin-page
```

## Goal
↪ Continues **TODO-19** — the admin endpoint, table and page. The data-table style rides along, closing **TODO-18 · Ch 10 · Data table** too. Append the test: Alice gets `403`; the admin gets every todo with its owner's email.

## Why
Here `403` is right: the admin page is not a secret, and an ordinary user is simply not allowed. `404` is for ids that would leak.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/admin.rs`.
