# Test 1: requests are counted

**File:** `crates/server/tests/observability.rs` (part 1 of 2)  
**Save to:** `~/code/todo/crates/server/tests/observability.rs`

## Goal
Write the test: after a request, `/metrics` has an `http_requests_total` line labelled with the route.

## Check
`bin/dev cargo t-http` — red: `server::metrics_router` does not exist.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/observability.rs`.
