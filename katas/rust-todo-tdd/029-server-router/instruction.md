# Green, part 3 — the server's router()

**File:** `crates/server/src/lib.rs` (part 1 of 5)  
**Save to:** `~/code/todo/crates/server/src/lib.rs`

## Goal
Add the server's dependencies and type `router()`: tests call it directly, `main` will serve it.

## Run first
```sh
$ bin/dev cargo add -p server app --path crates/app --features ssr
$ bin/dev cargo add -p server leptos@0.8 --features ssr
$ bin/dev cargo add -p server leptos_axum@0.8 axum@0.8
$ bin/dev cargo add -p server tokio@1 --features rt-multi-thread,macros
$ bin/dev cargo add -p server sqlx@0.8 --no-default-features --features runtime-tokio,postgres
$ bin/dev cargo add -p server --dev tower@0.5 --features util
$ bin/dev cargo add -p server --dev http-body-util@0.1
```

## Why
- `leptos_routes_with_context` registers every page route **and** every `#[server]` function, and runs the closure for each request: that is how `AppState` reaches server functions through `use_context`.
- The fallback serves static files from `target/site`, or renders the 404 page through the shell.
- The router lives in `lib.rs`, not `main.rs`, so the HTTP tests can build it without opening a port.

## Check
```sh
$ bin/dev cargo t-http      # green: test the_home_page_renders ... ok
```
The first compile takes a few minutes; later ones are incremental.

## Help
- Save with `:w! ~/code/todo/crates/server/src/lib.rs`.
