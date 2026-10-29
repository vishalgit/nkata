# Green, part 1 — AppState

**File:** `crates/app/src/state.rs`  
**Save to:** `~/code/todo/crates/app/src/state.rs`

## Goal
Add the `app` crate's dependencies and features, then type `AppState`: what every request handler may need.

## Run first
```sh
$ bin/dev cargo add -p app leptos@0.8 leptos_meta@0.8 leptos_router@0.8
$ bin/dev cargo add -p app axum@0.8 --optional --features macros
$ bin/dev cargo add -p app leptos_axum@0.8 --optional
$ bin/dev cargo add -p app sqlx@0.8 --optional --no-default-features --features runtime-tokio,postgres
```

Then edit `crates/app/Cargo.toml` by hand. `cargo add --optional` wrote a `[features]` table with one line per optional dependency (`axum = ["dep:axum"]`, …). Replace that whole table with these two features:

```toml
[features]
hydrate = ["leptos/hydrate"]
ssr = [
  "leptos/ssr", "leptos_meta/ssr", "leptos_router/ssr",
  "dep:axum", "dep:leptos_axum", "dep:sqlx",
]
```

## Why
- Server-only dependencies are `--optional`, and only `ssr` turns them on (`dep:axum`), so they never reach the WASM bundle.
- `#[derive(FromRef)]` lets an Axum handler ask for one field: `State<PgPool>` instead of the whole `AppState`.
- Server only: `lib.rs` declares this module under `#[cfg(feature = "ssr")]`.

## Help
- Save with `:w! ~/code/todo/crates/app/src/state.rs`.
