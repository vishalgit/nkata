# Test 2: an unknown id is not found

**File:** `crates/app/src/features/todos/store.rs` (part 3 of 21)  
**Save to:** `~/code/todo/crates/app/src/features/todos/store.rs`

## Goal
Append the test: `find` with an unknown id is `Ok(None)`.

## Why
Passes immediately: `fetch_optional` already returns `None`. Keep it — "not found" and "error" must stay different.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/store.rs`.
