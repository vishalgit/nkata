# Green — encoding belongs to a library

**File:** `crates/app/src/features/todos/list.rs` (part 19 of 25)  
**Save to:** `~/code/todo/crates/app/src/features/todos/list.rs`

## Goal
Add `serde_urlencoded` (it compiles to WASM too) and let it build the query string.

## Run first
```sh
$ bin/dev cargo add -p app serde_urlencoded@0.7
```

## Why
Hand-rolled URL encoding is a classic source of bugs. `serde_urlencoded` turns pairs into `a=1&b=x+y`, escaping everything that needs it.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/list.rs`.
