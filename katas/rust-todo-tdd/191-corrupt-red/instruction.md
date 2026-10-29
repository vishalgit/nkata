# Test 3: a row the domain rejects is reported, never trusted

**File:** `crates/app/src/features/todos/store.rs` (part 4 of 21)  
**Save to:** `~/code/todo/crates/app/src/features/todos/store.rs`

## Goal
Append the test: a row whose title the domain refuses comes back as `Corrupt`, not a panic.

## Why
Three spaces pass the SQL `CHECK` (length 3) but not `TodoTitle::parse`. The database and the domain disagree; the store must say so.

## Check
Red — it does not compile yet: there is no `Corrupt` variant. (Were there one, the `expect` would panic.)

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/store.rs`.
