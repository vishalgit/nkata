# The harness gets a real database

**File:** `crates/server/tests/common/mod.rs` (part 2 of 7)  
**Save to:** `~/code/todo/crates/server/tests/common/mod.rs`

**▶ Task**
```sh
scripts/backlog task TODO-2 2 http "/healthz and bacon (2.9–2.12)" healthz
```

## Goal
Kubernetes will ask "are you healthy?"; the honest answer includes the database. Give the harness an app connected to `DATABASE_URL`.

## Why
Inside the dev container `DATABASE_URL` points at the Compose database, so this helper needs no configuration.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/common/mod.rs`.
