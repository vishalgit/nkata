# Green — split on @

**File:** `crates/domain/src/identity/email.rs` (part 4 of 8)  
**Save to:** `~/code/todo/crates/domain/src/identity/email.rs`

## Goal
Split once on `@` and require both halves.

## Why
`split_once` returns `None` when there is no `@`; `.ok_or(…)?` turns that into the error and leaves.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/email.rs`.
