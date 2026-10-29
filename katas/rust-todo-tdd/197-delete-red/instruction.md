# Test 5: deleting removes the row exactly once

**File:** `crates/app/src/features/todos/store.rs` (part 9 of 21)  
**Save to:** `~/code/todo/crates/app/src/features/todos/store.rs`

**▶ Task**
```sh
scripts/backlog task TODO-13 5 persistence "delete in the store (5.7)" delete-store
```

## Goal
Append the test: the first delete reports a row gone, the second reports nothing to delete.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/store.rs`.
