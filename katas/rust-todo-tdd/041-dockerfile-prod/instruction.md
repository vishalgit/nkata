# The production image — Dockerfile

**File:** `Dockerfile` (part 1 of 3)  
**Save to:** `~/code/todo/Dockerfile`

**▶ Task**
```sh
scripts/backlog task TODO-3 2 infra "Production Dockerfile (2.13)" dockerfile
```

## Goal
Type the two-stage production `Dockerfile`: build as uid 1000, run `FROM scratch`.

## Why
- The server is compiled for `x86_64-unknown-linux-musl`, a fully static binary, so the final image can be `scratch`: no distribution, no shell, no package manager — just the binary, the static site and uid 1000.
- `LEPTOS_BIN_TARGET_TRIPLE` tells cargo-leptos to build the server for musl; the WASM is built for the browser as usual.
- In production there is no `Cargo.toml`, so `get_configuration(None)` reads the `LEPTOS_*` variables.

## Check
Build it and run it against the Compose database:
```sh
$ docker build -t todo:dev .
$ docker run --rm --network todo_default -p 8080:8080 \
    -e DATABASE_URL=postgres://todo:todo@db:5432/todo_dev todo:dev
$ curl -s localhost:8080/healthz        # in another window: ok
$ docker images todo:dev                # tiny
```

## When it passes
**◀ PR** `build: scratch production image (TODO-3)` · labels `type:task,layer:infra,epic:E0`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:infra,epic:E0
$ gh pr ready && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/Dockerfile`.
