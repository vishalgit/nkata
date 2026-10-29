# Test 9: a created todo is returned and listed

**File:** `crates/server/tests/todos.rs` (part 3 of 13)  
**Save to:** `~/code/todo/crates/server/tests/todos.rs`

**▶ Task**
```sh
TASK_ONLY=1 scripts/backlog task TODO-12 7 http "TodoPage from the first list test (7.9–7.10)"
TASK_ONLY=1 scripts/backlog task TODO-16 7 http "require_user and the owner-scoped list (7.8–7.9)"
scripts/backlog task TODO-17 7 http "List endpoint (7.9)" list-endpoint
```

## Goal
Import `TodoPage` and `TodoView`, then append the test: the created todo comes back trimmed, and the list is a page holding exactly it.

## Check
Red: no list endpoint, no `TodoPage`.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/todos.rs`.
