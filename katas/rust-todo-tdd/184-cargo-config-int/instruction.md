# The integration test layer arrives

**File:** `.cargo/config.toml` (part 4 of 4)  
**Save to:** `~/code/todo/.cargo/config.toml`

## Goal
Add the `t-int` alias: the `app` crate's tests, with `ssr`.

## Why
The store, and later the components, are tested inside `app`. They need `ssr` — and only `ssr`, never `hydrate` at the same time.

## Help
- Save with `:w! ~/code/todo/.cargo/config.toml`.
