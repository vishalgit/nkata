# Green — todo!() marks the next branch

**File:** `crates/domain/src/todos/workflows/complete.rs` (part 2 of 4)  
**Save to:** `~/code/todo/crates/domain/src/todos/workflows/complete.rs`

## Goal
Match on the state: an active todo completes; the completed branch is `todo!()` until a test asks for it.

## Why
`todo!()` compiles and panics if reached: an honest marker for "no test has asked for this yet".

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/workflows/complete.rs`.
