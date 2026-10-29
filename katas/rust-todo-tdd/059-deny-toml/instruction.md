# Dependency rules — deny.toml

**File:** `deny.toml`  
**Save to:** `~/code/todo/deny.toml`

## Goal
Mark the workspace crates private, then type `deny.toml`.

First add `publish = false` under `[package]` in `crates/app`, `crates/frontend` and `crates/server` `Cargo.toml` — and in every crate you create later.

## Why
- `private = { ignore = true }` skips licence checks for crates that are `publish = false`: your own.
- `multiple-versions = "warn"`: two versions of one crate is worth knowing, not worth failing over.
- Unknown registries and git sources are denied: every dependency comes from crates.io.

## Check
```sh
$ docker compose build dev && docker compose up -d dev
$ bin/dev cargo lint && bin/dev cargo deny check
```

## Help
- Save with `:w! ~/code/todo/deny.toml`.
