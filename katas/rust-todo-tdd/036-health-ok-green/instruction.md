# Green — the simplest thing

**File:** `crates/server/src/lib.rs` (part 2 of 5)  
**Save to:** `~/code/todo/crates/server/src/lib.rs`

## Goal
Add a `/healthz` route that answers `ok`.

## Why
The least code that passes. It ignores the database — the next test makes that a lie.

## Check
`bin/dev cargo t-http` — green.

## Help
- Save with `:w! ~/code/todo/crates/server/src/lib.rs`.
