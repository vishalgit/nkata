# The filter tabs

**File:** `crates/app/src/features/todos/list.rs` (part 23 of 25)  
**Save to:** `~/code/todo/crates/app/src/features/todos/list.rs`

## Goal
Three plain links; the current one is marked, and each keeps the search and returns to page 1. Add the component and its test.

## Why
- Plain `<a>` links work — the Leptos router intercepts them once hydrated — and without JavaScript they are ordinary navigation.
- The test expects `&amp;`: the server escapes `&` inside an attribute, as HTML requires.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/list.rs`.
