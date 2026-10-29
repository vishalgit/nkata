# Piece 1 — AppError

**File:** `crates/app/src/error.rs` (part 1 of 10)  
**Save to:** `~/code/todo/crates/app/src/error.rs`

## Goal
Add serde and tracing to `app`, then type the one error type every server function returns.

## Run first
```sh
$ bin/dev cargo add -p app serde@1 --features derive
$ bin/dev cargo add -p app tracing@0.1 --optional
```

Put `"dep:tracing"` in `ssr` (and delete the `tracing = ["dep:tracing"]` line cargo wrote).

## Why
- Each workflow has its own precise error; at the HTTP boundary they become one serialisable type. It starts with only what this test needs.
- `FromServerFnError` lets `#[server]` functions return `Result<T, AppError>` directly. Transport failures (a bad body, a network error in the browser) become `Transport`.
- `Unexpected` carries no text on purpose: a database error can reveal table names or data. The details go to the log.
- `status()` exists only on the server: the browser never sets a status.

## Help
- Save with `:w! ~/code/todo/crates/app/src/error.rs`.
