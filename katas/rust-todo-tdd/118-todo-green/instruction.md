# Green — TodoInfo, ActiveTodo, Todo

**File:** `crates/domain/src/todos/todo.rs` (part 2 of 10)  
**Save to:** `~/code/todo/crates/domain/src/todos/todo.rs`

## Goal
Add `chrono`, then the three types: the data every todo has, the active state, and an enum to hold any state.

## Run first
```sh
$ bin/dev cargo add -p domain chrono@0.4
```

## Why
- Time is a domain concept, so the domain may name it (`DateTime<Utc>`) — it just never reads the clock.
- `TodoInfo`'s fields are public because each one is already a constrained type: there is no invalid `TodoTitle` to put there.
- `is_completed` returns `false` because `Active` is the only state so far. The next test changes that.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/todo.rs`.
