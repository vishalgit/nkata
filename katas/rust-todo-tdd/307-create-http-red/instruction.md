# Test 8: creating a todo requires a login

**File:** `crates/server/tests/todos.rs` (part 1 of 13)  
**Save to:** `~/code/todo/crates/server/tests/todos.rs`

**▶ Task**
```sh
scripts/backlog task TODO-7 7 http "create_todo server function (7.8)" create-endpoint
```

## Goal
↪ Continues **TODO-7 · Add a todo** — the `create_todo` server function. Write the first todos test: anonymous creation is `401 Unauthenticated`.

## Why
No `TodoView` import yet: neither test in this file reads a todo back, and `cargo lint` denies an unused import in test code too. The list test brings it in.

## Check
Red. Green needs a variant, an effect, a DTO and the slice.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/todos.rs`.
