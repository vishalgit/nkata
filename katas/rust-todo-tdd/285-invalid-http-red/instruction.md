# Test 2: an invalid registration lists every problem

**File:** `crates/server/tests/auth.rs` (part 2 of 8)  
**Save to:** `~/code/todo/crates/server/tests/auth.rs`

## Goal
Append the test: a bad email and a short password answer `422` with both problems.

## Check
Red: there is no `Validation` variant yet (and the endpoint answers `500 Unexpected`).

## Help
- Save with `:w! ~/code/todo/crates/server/tests/auth.rs`.
