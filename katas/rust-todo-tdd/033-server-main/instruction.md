# A real main, and a look in the browser

**File:** `crates/server/src/main.rs` (part 1 of 3)  
**Save to:** `~/code/todo/crates/server/src/main.rs`

## Goal
Add logging and replace the hello-world `main.rs` with one that serves `router()`.

## Run first
```sh
$ bin/dev cargo add -p server tracing tracing-subscriber --features tracing-subscriber/env-filter
```

## Why
- `get_configuration(None)` reads the cargo-leptos settings in development and the `LEPTOS_*` environment variables in production (there is no `Cargo.toml` in the image).
- `connect_lazy`: the server starts even while Postgres is still booting. `/healthz` (next task) tells the truth about the database.
- `EnvFilter::from_default_env()` reads `RUST_LOG`, which `compose.yaml` sets to `info,sqlx=warn`.

## Check
```sh
$ docker compose up -d dev          # recreate with the new ports
$ bin/dev cargo leptos watch        # in the server window
```
Open `http://localhost:3000`: **Todo**. Edit the heading in `app.rs`, save, and the page reloads by itself. Change it back — the test depends on it.

## When it passes
**◀ PR** `feat: server-rendered, hydrated home page (TODO-2)` · labels `type:task,layer:http,epic:E0`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:http,epic:E0
$ gh pr ready && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/server/src/main.rs`.
