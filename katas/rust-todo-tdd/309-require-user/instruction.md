# Green, part 2 — require_user

**File:** `crates/app/src/effects.rs` (part 4 of 4)  
**Save to:** `~/code/todo/crates/app/src/effects.rs`

## Goal
One effect turns "nobody" into an error.

## Why
Every todo slice starts with `require_user()?`. It is the first half of TODO-16: no session, no todos.

## Help
- Save with `:w! ~/code/todo/crates/app/src/effects.rs`.
