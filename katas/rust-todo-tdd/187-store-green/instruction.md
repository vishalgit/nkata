# Green — the store

**File:** `crates/app/src/features/todos/store.rs` (part 2 of 21)  
**Save to:** `~/code/todo/crates/app/src/features/todos/store.rs`

## Goal
Add the dependencies, put `uuid/v4` in `ssr`, then type the DTO, `insert` and `find` above the tests.

## Run first
```sh
$ bin/dev cargo add -p app domain --path crates/domain
$ bin/dev cargo add -p app uuid@1 chrono@0.4 thiserror@2
$ bin/dev cargo add -p app sqlx --features macros,migrate,uuid,chrono
```

Check `sqlx` is still `optional = true` in `crates/app/Cargo.toml`, then add `"uuid/v4"` to the `ssr` list — id generation stays server-only:

```toml
ssr = [
  "leptos/ssr", "leptos_meta/ssr", "leptos_router/ssr",
  "dep:axum", "dep:leptos_axum", "dep:sqlx",
  "uuid/v4",
]
```

## Why
- The row struct is a **DTO**: it mirrors the table, not the domain. `into_domain` is where a row becomes a `Todo`.
- `query!` and `query_as!` check SQL **at compile time** against the live, migrated database — misspell a column and the build fails, not production.
- `.map(..).transpose()` turns `Option<Result<T, E>>` into `Result<Option<T>, E>`.
- The `expect` is a shortcut that test 3 removes.

## Check
`bin/dev cargo t-int` — green.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/store.rs`.
