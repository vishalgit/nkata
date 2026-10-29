# Green — LIMIT and OFFSET on the fetch only

**File:** `crates/app/src/features/todos/store.rs` (part 20 of 21)  
**Save to:** `~/code/todo/crates/app/src/features/todos/store.rs`

## Goal
Page the fetch; leave the count unpaged.

## Why
- The count stays unpaged: the pager needs the full total.
- The unique tiebreaker in `ORDER BY created_at DESC, id` matters: without it, rows with equal timestamps could appear on two pages or none.

## Check
Refresh `.sqlx/`: `bin/dev cargo sqlx prepare --workspace -- -p server`. It is part of the Definition of Done from now on. Then the chapter checkpoint:
```sh
$ bin/dev cargo t-unit && bin/dev cargo t-int && bin/dev cargo t-http
$ bin/deploy
$ kubectl -n todo exec statefulset/postgres -- psql -U todo -d todo -c '\dt'   # todos, _sqlx_migrations
```
The deployed pods ran the migrations themselves.

## When it passes
**◀ PR** `feat(store): page results ten at a time (TODO-12)` · labels `type:task,layer:persistence,epic:E2`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:persistence,epic:E2
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

**Checkpoint.** Close *Ch 05 · Persistence* (TODO-4's Chapter 5 task closed with it); merge the Release PR.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/store.rs`.
