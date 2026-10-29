# Green — no panic in a login path

**File:** `crates/app/src/features/auth/crypto.rs` (part 5 of 10)  
**Save to:** `~/code/todo/crates/app/src/features/auth/crypto.rs`

## Goal
A hash that does not parse simply does not verify.

## Why
A login endpoint must never panic on stored data: that would turn one corrupt row into a denial of service.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/crypto.rs`.
