# Green — Debug by hand

**File:** `crates/domain/src/identity/password.rs` (part 8 of 8)  
**Save to:** `~/code/todo/crates/domain/src/identity/password.rs`

## Goal
Remove `Debug` from the derive list and write it yourself.

## Why
`tracing::error!(?cmd)` or a failed `assert_eq!` prints `Debug`. A hand-written `Debug` makes every one of those safe.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/password.rs`.
