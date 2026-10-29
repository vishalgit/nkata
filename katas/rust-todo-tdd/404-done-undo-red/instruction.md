# Component test: Done for an active item, Undo for a completed one

**File:** `crates/app/src/features/todos/list.rs` (part 10 of 25)  
**Save to:** `~/code/todo/crates/app/src/features/todos/list.rs`

## Goal
Items will read actions from context, so the tests get a render helper that provides them. Add it and the test.

## Check
`bin/dev cargo t-int` — red: no buttons.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/list.rs`.
