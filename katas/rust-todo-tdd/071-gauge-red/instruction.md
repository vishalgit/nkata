# Test 2: database health is a metric

**File:** `crates/server/tests/observability.rs` (part 2 of 2)  
**Save to:** `~/code/todo/crates/server/tests/observability.rs`

## Goal
Append the test: after a health check, `/metrics` exports `app_database_up`.

## Check
`bin/dev cargo t-http` — red.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/observability.rs`.
