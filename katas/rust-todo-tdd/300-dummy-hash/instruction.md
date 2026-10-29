# A leak no functional test sees — timing

**File:** `crates/app/src/features/auth/crypto.rs` (part 10 of 10)  
**Save to:** `~/code/todo/crates/app/src/features/auth/crypto.rs`

## Goal
An unknown email returns *fast* (no Argon2), a known one *slowly*. Add a dummy hash to verify against when the email is unknown.

## Why
The response bodies are identical, but the response *times* differ by Argon2's whole cost — enough to tell which emails exist. `LazyLock` computes the dummy hash once, on first use.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/crypto.rs`.
