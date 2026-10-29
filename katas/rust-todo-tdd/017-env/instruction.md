# Host tools reach the database — .env

**File:** `.env` (part 1 of 2)  
**Save to:** `~/code/todo/.env`

## Goal
Type `.env`, then build and check the containers.

## Why
- Your editor and database tools run on the host and find the database here: your Neovim database plugins read `DATABASE_URL`, and from Chapter 5 rust-analyzer needs it to expand sqlx's `query!` macros.
- Inside the dev container, Compose's own `DATABASE_URL` (host `db`) wins.
- It is committed: it holds only dev settings.

## When it passes
Verify — this is the acceptance test for the first two criteria:

```sh
$ docker compose build dev          # first build takes several minutes
$ docker compose up -d db dev
$ docker compose exec db id         # uid=1000 gid=1000
$ docker compose exec dev id        # uid=1000(vishal) gid=1000(vishal)
$ docker compose exec dev cargo leptos --version
$ docker compose exec dev sqlx --version
$ docker compose exec dev rustup target list --installed   # wasm32-unknown-unknown, x86_64-unknown-linux-musl
$ ls -ld .pgdata                    # owned by vishal
$ psql "$(sed -n 's/^DATABASE_URL=//p' .env)" -c 'select 1'   # from the host, with your own client
```

**◀ PR** `build: dev container and database as uid 1000 (TODO-1)` · labels `type:task,layer:infra,epic:E0`

```sh
$ git add -A && git commit -m "build: dev container and database as uid 1000"
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:infra,epic:E0
$ gh pr ready && gh pr merge --squash --delete-branch
```
(The pipeline arrives in Chapter 2c; until then there are no checks to watch.)

## Help
- Save with `:w! ~/code/todo/.env`.
