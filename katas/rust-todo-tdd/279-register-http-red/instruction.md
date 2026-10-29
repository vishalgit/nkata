# Test 1: registering returns the user and starts a session

**File:** `crates/server/tests/auth.rs` (part 1 of 8)  
**Save to:** `~/code/todo/crates/server/tests/auth.rs`

## Goal
Write the first auth test: `POST /api/auth/register` answers `200` with the normalised user, and sets a session cookie.

## Check
**Red.** Green takes four pieces: a serialisable error, an effects module, the `UserView` DTO and the slice.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/auth.rs`.
