# Green — wire the middleware

**File:** `crates/server/src/lib.rs` (part 4 of 5)  
**Save to:** `~/code/todo/crates/server/src/lib.rs`

## Goal
Declare the module, re-export `metrics_router`, install the recorder first thing in `router()`, and wrap every route in `metrics::track`.

## Why
- `Router::layer` wraps every route **after** routing, so `MatchedPath` is available inside `track`.
- Installing the recorder before anything else means no early sample is lost.

## Check
`bin/dev cargo t-http` — green.

## Help
- Save with `:w! ~/code/todo/crates/server/src/lib.rs`.
