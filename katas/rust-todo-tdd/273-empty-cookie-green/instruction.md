# Green — filter the empty token

**File:** `crates/app/src/features/auth/cookie.rs` (part 8 of 8)  
**Save to:** `~/code/todo/crates/app/src/features/auth/cookie.rs`

## Goal
Append `.filter(|token| !token.is_empty())` to the chain.

## Why
The cookie a logout leaves behind is exactly `todo_session=`; it must identify nobody.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/cookie.rs`.
