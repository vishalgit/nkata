# workflows/mod.rs and its test fixtures

**File:** `crates/domain/src/todos/workflows/mod.rs` (part 1 of 5)  
**Save to:** `~/code/todo/crates/domain/src/todos/workflows/mod.rs`

## Goal
One module per workflow, plus fixtures their tests share.

## Why
- Fixtures grow with the tests that use them: `cargo lint` checks test code with `-D warnings`, so an unused fixture would fail the build.
- `#[cfg(test)] pub(crate)`: visible to every test in the crate, absent from the real build.
- Fixed times and fixed UUIDs make every assertion exact.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/workflows/mod.rs`.
