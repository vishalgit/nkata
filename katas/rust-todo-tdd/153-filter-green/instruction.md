# Green — a forgiving parser

**File:** `crates/domain/src/todos/filter.rs` (part 2 of 6)  
**Save to:** `~/code/todo/crates/domain/src/todos/filter.rs`

## Goal
An enum with a default, a parser that never fails, and `as_str` for building URLs.

## Why
- Unlike a title, a filter has a sensible fallback, so `parse` returns `Self`, not a `Result`: a bad URL never breaks the page.
- `#[default]` on `All` makes `Filter::default()` the same fallback.
- `use super::Todo;` is for the next test; until then the compiler warns it is unused.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/filter.rs`.
