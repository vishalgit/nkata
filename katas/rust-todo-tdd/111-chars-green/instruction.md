# Green — refactor into one match

**File:** `crates/domain/src/todos/title.rs` (part 12 of 15)  
**Save to:** `~/code/todo/crates/domain/src/todos/title.rs`

## Goal
Count characters once and decide in one `match`.

## Why
- `chars().count()` counts Unicode scalar values, not bytes.
- The match reads like the rule: zero is empty, over the maximum is too long, anything else is a title. A match guard (`n if n > …`) binds the count and tests it in one arm.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/title.rs`.
