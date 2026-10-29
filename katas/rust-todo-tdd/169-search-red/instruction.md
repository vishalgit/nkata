# Search test 1: an empty search means no search

**File:** `crates/domain/src/todos/query.rs` (part 7 of 14)  
**Save to:** `~/code/todo/crates/domain/src/todos/query.rs`

## Goal
"Nothing typed" is valid and means "show everything", so `parse` returns `Result<Option<SearchTerm>, _>`. Append the test: blanks are `Ok(None)`.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/query.rs`.
