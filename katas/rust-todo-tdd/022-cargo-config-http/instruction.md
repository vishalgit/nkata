# Three crates and the first test alias

**File:** `.cargo/config.toml` (part 1 of 4)  
**Save to:** `~/code/todo/.cargo/config.toml`

## Goal
Create the three crates, then type `.cargo/config.toml` with the first test alias.

## Run first
```sh
$ bin/dev cargo new --lib crates/app
$ bin/dev cargo new --lib crates/frontend
$ bin/dev cargo new crates/server
$ mkdir -p crates/server/tests/common
```

## Why
- `app` is compiled twice by cargo-leptos — with `ssr` for the server, `hydrate` for the browser. `frontend` and `server` are thin entry points. The pure `domain` crate joins in Chapter 3.

  ```text
  crates/
  ├── app/       # Leptos app, organised by vertical slices
  ├── frontend/  # WASM entry point: hydrate()
  └── server/    # Axum binary + router() that the HTTP tests call
  ```
- **Never** run `cargo build --workspace` or `cargo test --workspace` here: it compiles `app` with `ssr` and `hydrate` at once and fails. One alias per test layer picks the right crate and features; each alias arrives with its first test.

## Check
Empty the generated `lib.rs` files of `app` and `frontend` (delete the sample `add` function and its test). The `server` crate keeps its hello-world `main.rs` for now.

## When it passes
**◀ PR** `build: cargo workspace and crates (TODO-2)` · labels `type:task,layer:infra,epic:E0`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:infra,epic:E0
$ gh pr ready && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/.cargo/config.toml`.
