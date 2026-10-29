# Green, part 2 — access.rs

**File:** `crates/app/src/features/todos/access.rs` (part 1 of 3)  
**Save to:** `~/code/todo/crates/app/src/features/todos/access.rs`

## Goal
Four slices will need "load this todo for this user", so it lives in one feature-level helper.

## Why
It ignores the actor for now. The next-but-one test makes that a security bug — and this is the one place to fix it.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/access.rs`.
