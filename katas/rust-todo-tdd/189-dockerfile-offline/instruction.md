# The image builds without a database

**File:** `Dockerfile` (part 3 of 3)  
**Save to:** `~/code/todo/Dockerfile`

## Goal
In the builder stage, set `SQLX_OFFLINE=true` beside `LEPTOS_BIN_TARGET_TRIPLE`.

## Why
There is no Postgres inside `docker build`. With `SQLX_OFFLINE`, the `query!` macros read the committed `.sqlx/` files instead of asking a database.

## Help
- Save with `:w! ~/code/todo/Dockerfile`.
