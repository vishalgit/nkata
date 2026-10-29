# Test 9: the filter runs in SQL

**File:** `crates/app/src/features/todos/store.rs` (part 17 of 21)  
**Save to:** `~/code/todo/crates/app/src/features/todos/store.rs`

**▶ Task**
```sh
scripts/backlog task TODO-10 5 persistence "Filter condition in SQL (5.13)" sql-filter
```

## Goal
↪ Continues **TODO-10 · Filter by state**. Append the test: Active plus search `milk` finds only the open one.

## Check
Red: the filter is ignored, so the total is 2.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/store.rs`.
