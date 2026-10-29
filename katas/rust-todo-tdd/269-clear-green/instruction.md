# Green — clear_session_cookie

**File:** `crates/app/src/features/auth/cookie.rs` (part 4 of 8)  
**Save to:** `~/code/todo/crates/app/src/features/auth/cookie.rs`

## Goal
Same name, same path, no value, `Max-Age=0`.

## Why
A browser deletes a cookie only when name **and** path match, so the clearing cookie repeats `Path=/`.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/cookie.rs`.
