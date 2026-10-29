# Test 4: saving persists completion and title

**File:** `crates/app/src/features/todos/store.rs` (part 7 of 21)  
**Save to:** `~/code/todo/crates/app/src/features/todos/store.rs`

**▶ Task**
```sh
TASK_ONLY=1 scripts/backlog task TODO-9 5 persistence "save, shared with TODO-8 (5.6)"
scripts/backlog task TODO-8 5 persistence "save (5.6)" save
```

## Goal
↪ Continues **TODO-8 and TODO-9** — `save` persists completion and title (one PR closes both task issues). Append the test: complete, rename, save, find — the same todo comes back.

## Check
Red: no `save`, and rows never come back completed.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/store.rs`.
