# Test 1: completing a todo

**File:** `crates/server/tests/todos.rs` (part 7 of 13)  
**Save to:** `~/code/todo/crates/server/tests/todos.rs`

**▶ Task**
```sh
TASK_ONLY=1 scripts/backlog task TODO-16 9 http "access.rs: load only what you may touch (9.2)"
scripts/backlog task TODO-8 9 http "Complete and reopen slices (9.1, 9.3)" complete-reopen-slices
```

## Goal
↪ Continues **TODO-8** and **TODO-16** — one PR closes **TODO-8 · Ch 9 · Complete and reopen slices** and **TODO-16 · Ch 9 · access.rs**. The workflows and the store exist, so each slice is **one HTTP test → the slice → the next test**. Append the first.

## Why
The ownership check (TODO-16) ships in the same PR as the first slice that loads a todo by id. You will see the security bug red — but it never reaches `main`.

## Check
Red: no `/api/todos/complete`.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/todos.rs`.
