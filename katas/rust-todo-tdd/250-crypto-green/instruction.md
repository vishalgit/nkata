# Green — Argon2

**File:** `crates/app/src/features/auth/crypto.rs` (part 2 of 10)  
**Save to:** `~/code/todo/crates/app/src/features/auth/crypto.rs`

## Goal
Add `argon2` (server-only), make it fast in dev builds, then write the two functions.

## Run first
```sh
$ bin/dev cargo add -p app argon2@0.5 --optional
```

Put `"dep:argon2"` in the `ssr` list of `crates/app/Cargo.toml` (and delete the `argon2 = ["dep:argon2"]` line `cargo add` wrote to `[features]`).

Argon2 is deliberately slow — painfully so in debug builds. Optimise just that crate in dev and test builds; append to the root `Cargo.toml`:
```toml
[profile.dev.package.argon2]
opt-level = 3
```

## Why
- `hash_password` fits `register_user`'s `H`, `verify_password` fits `authenticate`'s `V` — dependency injection with function types.
- `Argon2::default()` is Argon2id with the recommended parameters. The PHC string stores the algorithm, parameters and salt alongside the hash, so `verify` needs nothing else.
- The `expect` is a shortcut; test 2 removes it.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/crypto.rs`.
