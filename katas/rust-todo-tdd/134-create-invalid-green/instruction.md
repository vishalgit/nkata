# Green — the railway switch

**File:** `crates/domain/src/todos/workflows/create.rs` (part 4 of 4)  
**Save to:** `~/code/todo/crates/domain/src/todos/workflows/create.rs`

## Goal
Give the error its variant and replace `expect` with `?`.

## Why
- Read the body as a pipeline: **validate → build → emit**.
- `?` is Rust's railway switch: on `Err` it leaves the happy track, converting `TitleError` into `CreateTodoError` through `#[from]`.
- `#[error(transparent)]` reuses the inner error's message: "title must not be empty".

## When it passes
**◀ PR** `feat(domain): create_todo workflow (TODO-7)` · labels `type:task,layer:domain,epic:E1`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:domain,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/workflows/create.rs`.
