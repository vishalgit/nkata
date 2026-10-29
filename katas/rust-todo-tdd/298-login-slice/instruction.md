# Green, part 2 — the login slice

**File:** `crates/app/src/features/auth/login.rs` (part 1 of 3)  
**Save to:** `~/code/todo/crates/app/src/features/auth/login.rs`

## Goal
Load credentials for the email (if it even parses), authenticate, start a session.

## Why
- An email that does not parse is treated like an unknown one: `None`, and `authenticate` gives the same error.
- `crypto::verify_password` is passed to `authenticate` as its `V`: the domain decides, the adapter verifies.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/login.rs`.
