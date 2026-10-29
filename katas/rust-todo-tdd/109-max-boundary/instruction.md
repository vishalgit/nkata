# A boundary test

**File:** `crates/domain/src/todos/title.rs` (part 10 of 15)  
**Save to:** `~/code/todo/crates/domain/src/todos/title.rs`

## Goal
Append `accepts_exactly_the_maximum_length` with `"a".repeat(MAX_TITLE_CHARS)`.

## Why
Off-by-one bugs live at boundaries: `>` versus `>=`. It passes, and now it stays that way.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/title.rs`.
