# Two documentation tests

**File:** `crates/domain/src/identity/register.rs` (part 5 of 5)  
**Save to:** `~/code/todo/crates/domain/src/identity/register.rs`

## Goal
Append two tests that pass immediately: invalid input is never hashed, and a hashing failure is reported.

## Why
The first proves the order of the pipeline — validate, *then* hash — with a fake that panics if called. Expensive work never runs on input that was going to be refused.

## When it passes
**◀ PR** `feat(identity): register_user with applicative validation (TODO-14)` · labels `type:task,layer:domain,epic:E3`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:domain,epic:E3
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/register.rs`.
