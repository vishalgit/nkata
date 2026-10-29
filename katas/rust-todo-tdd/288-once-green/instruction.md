# Green — Conflict is a 409

**File:** `crates/app/src/error.rs` (part 3 of 10)  
**Save to:** `~/code/todo/crates/app/src/error.rs`

## Goal
Add the variant, its status, and the one store error a user may see.

## Why
`EmailTaken` is a business answer the user can act on; every other store error is still logged and hidden.

## When it passes
**◀ PR** `feat(auth): register endpoint (TODO-14)` · labels `type:task,layer:http,epic:E3`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:http,epic:E3
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/app/src/error.rs`.
