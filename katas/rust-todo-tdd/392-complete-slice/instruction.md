# Green, part 3 — the complete slice

**File:** `crates/app/src/features/todos/complete.rs` (part 1 of 2)  
**Save to:** `~/code/todo/crates/app/src/features/todos/complete.rs`

## Goal
Require a user, load the todo for them, decide, save.

## Why
The domain's `complete_todo` refuses a completed todo; the `?` turns that into `409 Conflict` through the `From` impl.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/complete.rs`.
