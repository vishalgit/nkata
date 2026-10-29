# Authenticate — the right password

**File:** `crates/domain/src/identity/authenticate.rs` (part 1 of 3)  
**Save to:** `~/code/todo/crates/domain/src/identity/authenticate.rs`

## Goal
Write the test only: stored credentials plus the right password yield the actor, with its role.

## Why
`authenticate` takes what the shell loaded for the email — `Some(credentials)` or `None` — plus a verify function. No database, no Argon2.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/authenticate.rs`.
