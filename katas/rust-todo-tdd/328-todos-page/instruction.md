# Green, part 2 — the todos page guards itself

**File:** `crates/app/src/features/todos/page.rs` (part 1 of 5)  
**Save to:** `~/code/todo/crates/app/src/features/todos/page.rs`

## Goal
A page that shows the board to a user and redirects anyone else.

## Why
- `.into_any()` erases the two branches' different view types so one `match` can return either.
- The board is a heading for now; kata "board-green" fills it.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/page.rs`.
