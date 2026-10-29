# Test 2: the same email cannot register twice

**File:** `crates/app/src/features/auth/store.rs` (part 4 of 9)  
**Save to:** `~/code/todo/crates/app/src/features/auth/store.rs`

## Goal
Append the test: a second registration with the same address (in any case) is `EmailTaken`.

## Check
Red: no `EmailTaken` yet; the database answers with a generic error.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/store.rs`.
