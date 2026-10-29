# Run migrations on startup

**File:** `crates/server/src/main.rs` (part 3 of 3)  
**Save to:** `~/code/todo/crates/server/src/main.rs`

## Goal
In Kubernetes nobody runs `sqlx migrate run` by hand; the app migrates itself. Add sqlx's `migrate` and `macros` features to the server and run the migrations right after the pool is created.

## Run first
```sh
$ bin/dev cargo add -p server sqlx --features migrate,macros
```

## Why
- `migrate!` embeds the SQL into the binary at compile time: the `scratch` image needs no migration files.
- Postgres advisory locks make it safe for both replicas to start at once: one migrates, the other waits.
- The path is relative to `crates/server/Cargo.toml`.

## Help
- Save with `:w! ~/code/todo/crates/server/src/main.rs`.
