# Green — SHA-256

**File:** `crates/app/src/features/auth/crypto.rs` (part 9 of 10)  
**Save to:** `~/code/todo/crates/app/src/features/auth/crypto.rs`

## Goal
Add `sha2` (server-only) and hash the token.

## Run first
```sh
$ bin/dev cargo add -p app sha2@0.10 --optional
```

Put `"dep:sha2"` in `ssr` (and delete the `sha2 = ["dep:sha2"]` line cargo wrote).

## Why
A fast hash is right here, unlike for passwords: the token is already 256 random bits, so there is nothing to brute-force. Stable, so the cookie's hash finds its row.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/crypto.rs`.
