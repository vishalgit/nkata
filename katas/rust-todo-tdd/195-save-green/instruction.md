# Green — save, in four edits

**File:** `crates/app/src/features/todos/store.rs` (part 8 of 21)  
**Save to:** `~/code/todo/crates/app/src/features/todos/store.rs`

## Goal
Give the DTO the column, select it, pick the state from it, and add `save` with a small helper.

## Why
- A completed row becomes a `Todo` through the same `complete` transition as in the domain — the invariant cannot be skipped.
- `save` updates only what can change: the title and the completion time. Ids, owners and creation times never change.

## Check
`bin/dev cargo t-int` — green. Refresh `.sqlx/`: `bin/dev cargo sqlx prepare --workspace -- -p server`. It is part of the Definition of Done from now on.

## When it passes
**◀ PR** `feat(store): save completion and title (TODO-8, TODO-9)` · labels `type:task,layer:persistence,epic:E1` · closes both task issues

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:persistence,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/store.rs`.
