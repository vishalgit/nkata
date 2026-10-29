# Green — 32 random bytes as hex

**File:** `crates/app/src/features/auth/crypto.rs` (part 7 of 10)  
**Save to:** `~/code/todo/crates/app/src/features/auth/crypto.rs`

## Goal
Add `hex` (server-only), then fill 32 bytes from the operating system's RNG.

## Run first
```sh
$ bin/dev cargo add -p app hex@0.4 --optional
```

Put `"dep:hex"` in `ssr` (and delete the `hex = ["dep:hex"]` line cargo wrote).

## Why
256 bits from `OsRng`, the operating system's cryptographic RNG: unguessable, and long enough that a stolen database of hashes cannot be brute-forced back.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/crypto.rs`.
