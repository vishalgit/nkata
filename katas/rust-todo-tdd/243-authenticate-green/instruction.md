# Green — one error for every failure

**File:** `crates/domain/src/identity/authenticate.rs` (part 2 of 3)  
**Save to:** `~/code/todo/crates/domain/src/identity/authenticate.rs`

## Goal
Stored credentials, one error variant, and a two-arm `match`.

## Why
An unknown email and a wrong password must look identical, or attackers can probe which emails exist. The `_` arm covers both failures: there is no way for them to differ.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/authenticate.rs`.
