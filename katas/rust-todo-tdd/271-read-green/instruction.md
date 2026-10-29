# Green — read_session_token

**File:** `crates/app/src/features/auth/cookie.rs` (part 6 of 8)  
**Save to:** `~/code/todo/crates/app/src/features/auth/cookie.rs`

## Goal
Split on `;`, trim, strip the name and the `=`.

## Why
Stripping `todo_session` and then `=` separately means `todo_session_old=…` does not match: after the name comes `_`, not `=`.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/cookie.rs`.
