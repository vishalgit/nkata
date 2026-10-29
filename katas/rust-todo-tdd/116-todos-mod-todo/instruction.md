# Declare the todo module

**File:** `crates/domain/src/todos/mod.rs` (part 3 of 10)  
**Save to:** `~/code/todo/crates/domain/src/todos/mod.rs`

**▶ Task**
```sh
scripts/backlog task TODO-8 3 domain "One type per state (3.12–3.15)" todo-states
```

## Goal
A todo is **active** or **completed**, and a completed one has a completion time. Declare the module that says so.

## Why
Two types say exactly that; no `completed: bool` beside an `Option<DateTime>` that could disagree.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/mod.rs`.
