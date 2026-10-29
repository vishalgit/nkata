# A lint alias

**File:** `.cargo/config.toml` (part 2 of 4)  
**Save to:** `~/code/todo/.cargo/config.toml`

**▶ Task**
```sh
scripts/backlog task TODO-4 2 infra "pipeline.yml (2.18)" pipeline
```

## Goal
The pipeline needs three things first. The first: append a `lint` alias.

## Why
- `-D warnings` turns every clippy warning into an error, in test code too (`--all-targets`).
- Only `server` for now; the domain crate joins it in Chapter 3.

## Check
```sh
$ bin/dev cargo lint
```

## Help
- Save with `:w! ~/code/todo/.cargo/config.toml`.
