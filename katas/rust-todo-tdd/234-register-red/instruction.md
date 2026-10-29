# Register — test 1: a valid registration

**File:** `crates/domain/src/identity/register.rs` (part 1 of 5)  
**Save to:** `~/code/todo/crates/domain/src/identity/register.rs`

## Goal
Write the test only. The hashing function is a **dependency passed as a parameter**, so the test supplies a fake.

## Why
The fake "hash" just prefixes the password: the test can see exactly what was hashed, and runs in microseconds instead of Argon2's deliberate slowness.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/register.rs`.
