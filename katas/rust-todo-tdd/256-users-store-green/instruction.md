# Green — insert_user and find_credentials

**File:** `crates/app/src/features/auth/store.rs` (part 2 of 9)  
**Save to:** `~/code/todo/crates/app/src/features/auth/store.rs`

## Goal
Insert a registered user; load credentials by email.

## Why
- `find_credentials` returns exactly `authenticate`'s input type, `Option<StoredCredentials>`: the shell passes it straight through.
- An unknown role in the database is `Corrupt`, never a guess.
- No `chrono` import at the top yet: only the tests need times so far, and `cargo lint` denies an unused import. The session functions bring it in.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/store.rs`.
