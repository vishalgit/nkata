# Green — translate the unique violation

**File:** `crates/app/src/features/auth/store.rs` (part 5 of 9)  
**Save to:** `~/code/todo/crates/app/src/features/auth/store.rs`

## Goal
Map Postgres's unique violation to `EmailTaken`.

## Why
- `ALICE@` collides because `EmailAddress::parse` lower-cased it before it reached the `UNIQUE` index.
- The database is the only place that can check uniqueness without a race; the store's job is to translate its answer into the domain's language.

## Check
Refresh `.sqlx/`: `bin/dev cargo sqlx prepare --workspace -- -p server`. It is part of the Definition of Done from now on.

## When it passes
**◀ PR** `feat(auth): users table, Argon2 hashing and the users store (TODO-14)` · labels `type:task,layer:persistence,epic:E3`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:persistence,epic:E3
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/store.rs`.
