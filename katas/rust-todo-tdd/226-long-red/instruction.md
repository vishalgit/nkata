# Test 3: too long

**File:** `crates/domain/src/identity/password.rs` (part 5 of 8)  
**Save to:** `~/code/todo/crates/domain/src/identity/password.rs`

## Goal
Append the test: 129 characters is `TooLong { max: 128 }`.

## Why
A maximum protects the server: Argon2 on a megabyte-long "password" is a denial of service.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/password.rs`.
