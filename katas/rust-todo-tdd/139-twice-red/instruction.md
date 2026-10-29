# Complete — test 2: completing twice is a business error

**File:** `crates/domain/src/todos/workflows/complete.rs` (part 3 of 4)  
**Save to:** `~/code/todo/crates/domain/src/todos/workflows/complete.rs`

## Goal
Append the test: completing a completed todo is `Err(AlreadyCompleted)`.

## Check
Red: no `AlreadyCompleted` variant yet. (With one, the test would hit `todo!()` and panic with "not yet implemented: completing twice".)

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/workflows/complete.rs`.
