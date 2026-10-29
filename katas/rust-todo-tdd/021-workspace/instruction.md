# The workspace — root Cargo.toml

**File:** `Cargo.toml` (part 1 of 2)  
**Save to:** `~/code/todo/Cargo.toml`

**▶ Task**
```sh
scripts/backlog task TODO-2 2 infra "Workspace, crates and test aliases (2.1–2.3)" workspace
```

## Goal
Start the task and type the root `Cargo.toml`: a workspace of every crate under `crates/`.

## Why
- No `[workspace.dependencies]`: each crate gets its dependencies with `cargo add`, when a test needs them.
- `resolver = "3"` is the edition-2024 resolver; it also respects `rust-version` when picking versions.
- The cargo-leptos settings join this file when you first run cargo-leptos (kata "cargo-leptos settings").

## Help
- Save with `:w! ~/code/todo/Cargo.toml`.
