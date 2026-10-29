# Test 7: logging out ends the session

**File:** `crates/server/tests/auth.rs` (part 8 of 8)  
**Save to:** `~/code/todo/crates/server/tests/auth.rs`

## Goal
Append the test: after logout, the **same** cookie identifies nobody.

## Why
Clearing the browser's cookie is not enough: the session row must go, or a copied cookie would still work.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/auth.rs`.
