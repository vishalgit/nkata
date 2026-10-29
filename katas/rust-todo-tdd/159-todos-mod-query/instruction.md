# Declare the query module

**File:** `crates/domain/src/todos/mod.rs` (part 7 of 10)  
**Save to:** `~/code/todo/crates/domain/src/todos/mod.rs`

**▶ Task**
```sh
scripts/backlog task TODO-12 4 domain "PageRequest and Paged<T> (4.14–4.16)" paging
```

## Goal
The list query's types — paging now, search next — live in one module. Declare it and export the page request.

## Why
Export only what exists by the end of the next green. (Exporting `Paged` now, before it is written, would keep the next green from compiling.)

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/mod.rs`.
