# Test 2: a garbage hash never verifies

**File:** `crates/app/src/features/auth/crypto.rs` (part 4 of 10)  
**Save to:** `~/code/todo/crates/app/src/features/auth/crypto.rs`

## Goal
Append the test: a stored value that is not a PHC string verifies nothing.

## Check
Red: the `expect` panics.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/crypto.rs`.
