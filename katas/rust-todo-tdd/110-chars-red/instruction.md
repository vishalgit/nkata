# Test 5: characters, not bytes

**File:** `crates/domain/src/todos/title.rs` (part 11 of 15)  
**Save to:** `~/code/todo/crates/domain/src/todos/title.rs`

## Goal
Append the test: 120 `é` characters are a valid title.

## Check
**Red** — `len()` counts bytes. The test proves the bug before you fix it.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/title.rs`.
