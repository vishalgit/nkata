# Green — a business error, not a panic

**File:** `crates/domain/src/todos/workflows/complete.rs` (part 4 of 4)  
**Save to:** `~/code/todo/crates/domain/src/todos/workflows/complete.rs`

## Goal
Add the variant and return it from the second arm.

## Why
Completing twice is not a bug in the program; it is a request the business rules refuse. So it is a value in the error type, and later an HTTP `409`.

Add a third state to `Todo` one day and the compiler lists every workflow `match` to revisit.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/workflows/complete.rs`.
