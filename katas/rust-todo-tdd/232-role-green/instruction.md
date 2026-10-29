# Green — Role

**File:** `crates/domain/src/identity/role.rs` (part 2 of 3)  
**Save to:** `~/code/todo/crates/domain/src/identity/role.rs`

## Goal
An enum and its storage names.

## Why
`parse` returns `Option`, not a fallback: an unknown role in the database is corruption, and the store will say so.

## When it passes
**◀ PR** `feat(identity): email, password and role types (TODO-14)` · labels `type:task,layer:domain,epic:E3`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:domain,epic:E3
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/role.rs`.
