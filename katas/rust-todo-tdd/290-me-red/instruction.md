# Test 4: the session cookie identifies you

**File:** `crates/server/tests/auth.rs` (part 4 of 8)  
**Save to:** `~/code/todo/crates/server/tests/auth.rs`

## Goal
Append the test: `POST /api/auth/me` with the cookie returns the user.

## Check
Red: no `/api/auth/me` (the 404 page is not JSON).

## Help
- Save with `:w! ~/code/todo/crates/server/tests/auth.rs`.
