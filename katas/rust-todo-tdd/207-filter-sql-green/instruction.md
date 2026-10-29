# Green — one more condition, twice

**File:** `crates/app/src/features/todos/store.rs` (part 18 of 21)  
**Save to:** `~/code/todo/crates/app/src/features/todos/store.rs`

## Goal
Add the completion condition to both queries, with `query.filter.completed()` as the third parameter.

## Why
`Filter::completed()` returns `Option<bool>` precisely for this: `None` switches the condition off, `Some(b)` compares.

## Check
Refresh `.sqlx/`: `bin/dev cargo sqlx prepare --workspace -- -p server`. It is part of the Definition of Done from now on.

## When it passes
**◀ PR** `feat(store): filter by completion state in SQL (TODO-10)` · labels `type:task,layer:persistence,epic:E2`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:persistence,epic:E2
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/store.rs`.
