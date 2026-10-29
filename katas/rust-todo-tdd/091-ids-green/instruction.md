# Green — TodoId

**File:** `crates/domain/src/shared/ids.rs` (part 2 of 6)  
**Save to:** `~/code/todo/crates/domain/src/shared/ids.rs`

## Goal
Add `uuid`, then the newtype above the tests.

## Run first
```sh
$ bin/dev cargo add -p domain uuid@1
```

## Why
- A newtype around `Uuid`: a `TodoId` and a raw `Uuid` are different types, so you cannot pass one where the other is meant.
- `Copy`: a UUID is 16 bytes; copying is cheaper than borrowing.
- The domain does **not** generate UUIDs. Randomness is an effect; the shell creates ids and passes them in.

## Check
After `cargo add` the test is still red (no `TodoId`). After this kata: green.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/shared/ids.rs`.
