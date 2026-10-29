# Create — test 2: an invalid title is reported, not panicked

**File:** `crates/domain/src/todos/workflows/create.rs` (part 3 of 4)  
**Save to:** `~/code/todo/crates/domain/src/todos/workflows/create.rs`

## Goal
Append the test: a blank title is `Err(CreateTodoError::InvalidTitle(TitleError::Empty))`.

## Check
Red — it does not even compile: there is no `InvalidTitle` variant, and `TitleError` is not imported. Were it to compile, the `expect` would panic.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/workflows/create.rs`.
