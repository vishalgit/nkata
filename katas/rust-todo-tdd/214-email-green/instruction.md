# Green — normalise

**File:** `crates/domain/src/identity/email.rs` (part 2 of 8)  
**Save to:** `~/code/todo/crates/domain/src/identity/email.rs`

## Goal
A newtype, an error, and a `parse` that trims and lower-cases.

## Why
- Normalising in the smart constructor means `Alice@Example.com` and `alice@example.com` are the same `EmailAddress` everywhere — including in the `UNIQUE` index later.
- `From<EmailAddress> for String` gives the inner string back when something needs to own it.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/email.rs`.
