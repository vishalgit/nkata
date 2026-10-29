# HTTP test 1: an admin can delete anyone's todo

**File:** `crates/server/tests/admin.rs` (part 1 of 4)  
**Save to:** `~/code/todo/crates/server/tests/admin.rs`

## Goal
Create `admin.rs` with a shared `setup` and the first test.

## Check
Red: `404` — the inline ownership check knows nothing about roles.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/admin.rs`.
