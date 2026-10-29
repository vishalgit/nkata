# Green — Pager

**File:** `crates/app/src/features/todos/list.rs` (part 21 of 25)  
**Save to:** `~/code/todo/crates/app/src/features/todos/list.rs`

## Goal
Previous and Next only where they lead somewhere, and a status in between.

## Why
- `bool::then` makes an `Option` from a condition: no link when there is nowhere to go.
- `rel="prev"` / `rel="next"` tell browsers and assistive tech how pages relate.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/list.rs`.
