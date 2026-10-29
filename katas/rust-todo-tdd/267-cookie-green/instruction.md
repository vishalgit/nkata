# Green — session_cookie

**File:** `crates/app/src/features/auth/cookie.rs` (part 2 of 8)  
**Save to:** `~/code/todo/crates/app/src/features/auth/cookie.rs`

## Goal
Two constants and one `format!`.

## Why
- `HttpOnly`: JavaScript cannot read it, so an XSS bug cannot steal it.
- `SameSite=Lax`: other sites cannot make your browser send it with a cross-site POST.
- `Secure` only in production: browsers refuse `Secure` cookies over plain http, except on `localhost`.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/cookie.rs`.
