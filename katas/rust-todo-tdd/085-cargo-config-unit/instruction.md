# The domain crate joins the build

**File:** `.cargo/config.toml` (part 3 of 4)  
**Save to:** `~/code/todo/.cargo/config.toml`

**▶ Task**
```sh
scripts/backlog task TODO-7 3 domain "Typed ids and the constrained title (3.2–3.11)" ids-and-title
```

## Goal
Create the `domain` crate, then teach the aliases about the new test layer: `t-unit`, and `lint` now covers `domain` too.

## Run first
```sh
$ bin/dev cargo new --lib crates/domain
```

Empty the generated `crates/domain/src/lib.rs`, and add `publish = false` under `[package]` in `crates/domain/Cargo.toml`.

## Why
- `domain` is the functional core: pure types and functions, no database, no web framework, no clock. It is a separate crate so the compiler enforces that — it simply cannot depend on `sqlx`.
- `t-unit` is its test layer. It compiles in seconds, so it is the one you will run most.

## Help
- Save with `:w! ~/code/todo/.cargo/config.toml`.
