# Services — compose.yaml

**File:** `compose.yaml` (part 1 of 3)  
**Save to:** `~/code/todo/compose.yaml`

## Goal
Type `compose.yaml`: Postgres and the dev container, both as uid 1000.

## Why
- The Postgres image runs as any uid whose data directory is writable — that is what the bind-mounted `.pgdata` gives it.
- Mounting `/var/lib/postgresql` (not `…/data`) matches current Postgres images.
- `127.0.0.1:5432` publishes the database to your host only, for vim-dadbod and rust-analyzer.
- The named volumes keep the cargo registry and caches between container restarts.
- The dev server's ports join this file in Chapter 2, and the `e2e` service after that.

## Help
- Save with `:w! ~/code/todo/compose.yaml`.
