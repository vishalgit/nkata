# Green — rows go through the smart constructors

**File:** `crates/app/src/features/todos/store.rs` (part 5 of 21)  
**Save to:** `~/code/todo/crates/app/src/features/todos/store.rs`

## Goal
Add the variant and map the parse error into it.

## Why
Rows go through the same smart constructors as user input. A bad row is reported with its id and the reason, and the rest of the program never sees an invalid `TodoTitle`.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/store.rs`.
