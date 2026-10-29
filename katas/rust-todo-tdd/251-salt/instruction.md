# A test that passes for free

**File:** `crates/app/src/features/auth/crypto.rs` (part 3 of 10)  
**Save to:** `~/code/todo/crates/app/src/features/auth/crypto.rs`

## Goal
Append the test: the same password hashed twice gives two different hashes.

## Why
Every hash gets a fresh salt, so two users with the same password have different hashes — and a precomputed table is useless.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/crypto.rs`.
