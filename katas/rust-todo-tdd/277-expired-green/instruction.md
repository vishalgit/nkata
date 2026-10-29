# Green — expiry and delete

**File:** `crates/app/src/features/auth/store.rs` (part 9 of 9)  
**Save to:** `~/code/todo/crates/app/src/features/auth/store.rs`

## Goal
Compare `expires_at` with `now` in SQL, and add `delete_session`.

## Why
`now` is a parameter, not SQL's `now()`: the store stays testable at any time you choose (here, 10:00 on a fixed day).

## Check
Refresh `.sqlx/`: `bin/dev cargo sqlx prepare --workspace -- -p server`. It is part of the Definition of Done from now on.

## When it passes
**◀ PR** `feat(auth): hashed session tokens, cookies and the session store (TODO-15)` · labels `type:task,layer:persistence,epic:E3`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:persistence,epic:E3
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

**Checkpoint.** `bin/dev cargo t-unit && bin/dev cargo t-int && bin/dev cargo t-http` green. Close *Ch 06 · Identity*; merge the Release PR.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/store.rs`.
