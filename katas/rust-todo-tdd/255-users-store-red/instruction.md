# The users store — test 1: found by email

**File:** `crates/app/src/features/auth/store.rs` (part 1 of 9)  
**Save to:** `~/code/todo/crates/app/src/features/auth/store.rs`

## Goal
Write the test only: a registered user can be found by email, with id, role and hash.

## Why
The test builds the row through the real `register_user` workflow, with a fake hash: the store stores what the domain produces.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/store.rs`.
