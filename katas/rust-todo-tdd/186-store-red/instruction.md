# Store test 1: an inserted todo can be found again

**File:** `crates/app/src/features/todos/store.rs` (part 1 of 21)  
**Save to:** `~/code/todo/crates/app/src/features/todos/store.rs`

## Goal
Write the test only. `#[sqlx::test]` creates a fresh database per test, applies every migration, hands the test a pool, and drops it afterwards.

## Why
Tests run in parallel without seeing each other's rows. The SQL you test is the SQL you ship.

## Check
`bin/dev cargo t-int` — **red**, and the compiler names the missing pieces: `domain`, `uuid`, `chrono`, `sqlx::test`.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/store.rs`.
