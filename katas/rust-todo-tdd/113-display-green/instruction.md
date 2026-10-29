# Green — thiserror

**File:** `crates/domain/src/todos/title.rs` (part 14 of 15)  
**Save to:** `~/code/todo/crates/domain/src/todos/title.rs`

## Goal
Add `thiserror` and derive `Error` with one message per variant.

## Run first
```sh
$ bin/dev cargo add -p domain thiserror@2
```

## Why
`thiserror` writes the `Display` and `std::error::Error` impls from the `#[error(…)]` attributes; `{max}` and `{actual}` refer to the variant's fields.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/title.rs`.
