# A schema test, against raw SQL

**File:** `crates/app/src/features/todos/store.rs` (part 6 of 21)  
**Save to:** `~/code/todo/crates/app/src/features/todos/store.rs`

## Goal
Append a test that bypasses the domain and inserts 121 characters directly.

## Why
- It passes because the `CHECK` constraint does its job — and it will catch the day someone drops it.
- Postgres stores microseconds and `Utc::now()` has nanoseconds; these tests use whole hours so round-trips compare equal.

## Check
```sh
$ bin/dev cargo t-int
$ bin/dev cargo sqlx prepare --workspace -- -p server     # commit .sqlx/
```

## When it passes
**◀ PR** `feat(store): todos table, insert and find (TODO-7)` · labels `type:task,layer:persistence,epic:E1` · body: `Closes #<TODO-7 Ch 5>, closes #<TODO-4 Ch 5>`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:persistence,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/store.rs`.
