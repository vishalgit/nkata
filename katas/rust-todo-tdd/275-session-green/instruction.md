# Green — SessionUser, create and find

**File:** `crates/app/src/features/auth/store.rs` (part 7 of 9)  
**Save to:** `~/code/todo/crates/app/src/features/auth/store.rs`

## Goal
The session's user type, an insert, and a join that resolves a token hash to a user.

## Why
- Every request will re-read the user's role through this join, so demoting an admin takes effect on their next click.
- Even the email from the database goes through `EmailAddress::parse`.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/store.rs`.
