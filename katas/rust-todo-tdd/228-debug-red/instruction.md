# Test 4: a password never shows up in logs

**File:** `crates/domain/src/identity/password.rs` (part 7 of 8)  
**Save to:** `~/code/todo/crates/domain/src/identity/password.rs`

## Goal
Append the test: `{:?}` prints `Password(***)`.

## Check
Red: the derived `Debug` prints the secret.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/password.rs`.
