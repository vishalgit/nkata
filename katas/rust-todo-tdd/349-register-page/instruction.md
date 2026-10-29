# Part 2 — RegisterPage joins its slice

**File:** `crates/app/src/features/auth/register.rs` (part 2 of 2)  
**Save to:** `~/code/todo/crates/app/src/features/auth/register.rs`

## Goal
Append the page to `register.rs`.

## Why
`minlength="12"` is a courtesy that matches the domain rule; the server checks it anyway. A validation error comes back as `AppError::Validation` and the `ErrorMessage` shows every problem at once.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/register.rs`.
