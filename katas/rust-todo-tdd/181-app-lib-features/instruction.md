# Where persistence lives in a vertical-slice app

**File:** `crates/app/src/lib.rs` (part 2 of 4)  
**Save to:** `~/code/todo/crates/app/src/lib.rs`

## Goal
Declare `features`: one folder per feature, one file per slice.

## Run first
```sh
$ mkdir -p crates/app/src/features/todos
```

## Why
SQL used by **one** slice stays in that slice; SQL used by several moves up to the feature's `store.rs`. Nothing crosses feature boundaries. Loading and saving a whole `Todo` serves several slices, so it starts in `features/todos/store.rs`.

## Help
- Save with `:w! ~/code/todo/crates/app/src/lib.rs`.
