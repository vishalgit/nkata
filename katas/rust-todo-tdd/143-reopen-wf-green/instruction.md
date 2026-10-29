# Reopen — compare

**File:** `crates/domain/src/todos/workflows/reopen.rs` (part 2 of 2)  
**Save to:** `~/code/todo/crates/domain/src/todos/workflows/reopen.rs`

## Goal
The code after both tests. Compare it with yours.

## Why
No clock: reopening records no time, so the workflow takes no `now`. Dependencies are exactly what the decision needs.

## When it passes
**◀ PR** `feat(domain): complete and reopen workflows (TODO-8)` · labels `type:task,layer:domain,epic:E1`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:domain,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/workflows/reopen.rs`.
