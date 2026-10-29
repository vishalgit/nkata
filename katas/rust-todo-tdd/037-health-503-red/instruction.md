# Red — health when the database is down

**File:** `crates/server/tests/skeleton.rs` (part 3 of 4)  
**Save to:** `~/code/todo/crates/server/tests/skeleton.rs`

## Goal
Append the test: with no database, `/healthz` answers `503`.

## Check
`bin/dev cargo t-http` — red: the fake answers `200`.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/skeleton.rs`.
