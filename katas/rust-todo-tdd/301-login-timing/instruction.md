# Refactor — both paths take the same time

**File:** `crates/app/src/features/auth/login.rs` (part 2 of 3)  
**Save to:** `~/code/todo/crates/app/src/features/auth/login.rs`

## Goal
In the login shell, just before `authenticate`, verify against the dummy hash when nothing was stored.

## Check
Still green: the refactor changed timing, not behaviour.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/login.rs`.
