# Red — health when the database answers

**File:** `crates/server/tests/skeleton.rs` (part 2 of 4)  
**Save to:** `~/code/todo/crates/server/tests/skeleton.rs`

## Goal
Append the test: `GET /healthz` answers `200 ok` when Postgres is up.

## Check
```sh
$ bin/dev cargo t-http      # red: 404, no such route
```

## Help
- Save with `:w! ~/code/todo/crates/server/tests/skeleton.rs`.
