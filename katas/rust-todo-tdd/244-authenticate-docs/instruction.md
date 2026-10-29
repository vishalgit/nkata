# Pin it down

**File:** `crates/domain/src/identity/authenticate.rs` (part 3 of 3)  
**Save to:** `~/code/todo/crates/domain/src/identity/authenticate.rs`

## Goal
Append two tests that pass immediately: a wrong password, and an unknown user, get the very same error.

## When it passes
**◀ PR** `feat(identity): authenticate with one error for every failure (TODO-15)` · labels `type:task,layer:domain,epic:E3`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:domain,epic:E3
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/authenticate.rs`.
