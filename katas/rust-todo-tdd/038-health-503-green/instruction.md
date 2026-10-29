# Green — ask the database

**File:** `crates/server/src/lib.rs` (part 3 of 5)  
**Save to:** `~/code/todo/crates/server/src/lib.rs`

## Goal
Replace the handler: run `SELECT 1` and answer `503` when it fails.

## Why
- `State<PgPool>` works because `#[derive(FromRef)]` on `AppState` lets Axum pull one field out.
- The warning goes to the logs with the real error; the response says only "database unavailable".

## Check
`bin/dev cargo t-http` — all three tests green.

## Help
- Save with `:w! ~/code/todo/crates/server/src/lib.rs`.
