# Green, part 3 — TodoView

**File:** `crates/app/src/features/todos/view.rs` (part 1 of 2)  
**Save to:** `~/code/todo/crates/app/src/features/todos/view.rs`

## Goal
Give `uuid` serde support, then type what the browser receives for one todo.

## Run first
```sh
$ bin/dev cargo add -p app uuid --features serde
```

## Why
The browser does not get the owner or the timestamps: a DTO carries only what the page shows. The domain types never leave the server.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/view.rs`.
