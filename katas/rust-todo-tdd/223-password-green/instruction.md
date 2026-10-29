# Green — a secret with one door

**File:** `crates/domain/src/identity/password.rs` (part 2 of 8)  
**Save to:** `~/code/todo/crates/domain/src/identity/password.rs`

## Goal
A newtype whose only reader is `expose_secret`.

## Why
- Passwords are never trimmed: spaces are legitimate password characters.
- Not `Serialize`, not `Display`: there is no accidental way to send it anywhere.
- `expose_secret` is deliberately loud, so every place that reads a password is easy to find.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/password.rs`.
