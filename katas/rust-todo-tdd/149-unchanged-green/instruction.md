# Green — an explicit pipeline

**File:** `crates/domain/src/todos/workflows/rename.rs` (part 5 of 5)  
**Save to:** `~/code/todo/crates/domain/src/todos/workflows/rename.rs`

## Goal
Add the variant, then refactor into a pipeline of three small steps.

## Why
- `map` transforms the success track, `and_then` adds a step that may fail, `map_err` lifts errors onto the workflow's error type — DMMF's railway combinators.
- The `?` style in `create_todo` is equivalent; use whichever reads better.
- Comparing `TodoTitle`s, not strings: both went through `parse`, so `"  Buy milk "` already became `"Buy milk"`.

## When it passes
**◀ PR** `feat(domain): rename_todo pipeline (TODO-9)` · labels `type:task,layer:domain,epic:E1`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:domain,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/workflows/rename.rs`.
