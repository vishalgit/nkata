# An `active` fixture, and the complete module

**File:** `crates/domain/src/todos/workflows/mod.rs` (part 2 of 5)  
**Save to:** `~/code/todo/crates/domain/src/todos/workflows/mod.rs`

**▶ Task**
```sh
scripts/backlog task TODO-8 4 domain "complete_todo and reopen_todo (4.5–4.7)" complete-reopen
```

## Goal
↪ Continues **TODO-8** — the `complete_todo` and `reopen_todo` workflows. Declare `complete` and add the first fixture that builds a whole todo.

## Why
`complete` is the first workflow that needs an existing todo, so `active(title)` arrives with it.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/workflows/mod.rs`.
