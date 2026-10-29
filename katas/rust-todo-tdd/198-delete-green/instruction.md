# Green — delete

**File:** `crates/app/src/features/todos/store.rs` (part 10 of 21)  
**Save to:** `~/code/todo/crates/app/src/features/todos/store.rs`

## Goal
Delete by id and report whether a row went.

## Why
The `bool` lets the HTTP layer answer `404` for the second delete without a second query.

## Check
Refresh `.sqlx/`: `bin/dev cargo sqlx prepare --workspace -- -p server`. It is part of the Definition of Done from now on.

## When it passes
**◀ PR** `feat(store): delete a todo (TODO-13)` · labels `type:task,layer:persistence,epic:E1`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:persistence,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/store.rs`.
