# Two documentation tests

**File:** `crates/domain/src/todos/workflows/rename.rs` (part 3 of 5)  
**Save to:** `~/code/todo/crates/domain/src/todos/workflows/rename.rs`

## Goal
Append two tests that pass immediately: a completed todo stays completed, and a blank title is rejected.

## Why
They drove no code — `with_title` and `TodoTitle::parse` already do the work — but they pin down two acceptance criteria at the domain layer.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/workflows/rename.rs`.
