# Green, part 1 — the login action lives in the context

**File:** `crates/app/src/features/auth/context.rs` (part 2 of 4)  
**Save to:** `~/code/todo/crates/app/src/features/auth/context.rs`

## Goal
The login action belongs to the shared auth context, and `me` must refetch when it finishes.

## Why
`login.version()` increments every time the action completes. Using it as the resource's source makes `me()` run again after each login, so the header updates without a page load.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/context.rs`.
