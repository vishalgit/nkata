# A test that passes immediately

**File:** `crates/domain/src/todos/title.rs` (part 7 of 15)  
**Save to:** `~/code/todo/crates/domain/src/todos/title.rs`

## Goal
Append `rejects_a_whitespace_only_title` with `"   \t"`.

## Why
It passes at once — the code already handles it. Keep it anyway: it documents a rule someone might break while "simplifying" `parse`. It drove no code.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/title.rs`.
