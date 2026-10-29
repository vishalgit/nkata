# Green, part 3 — the reopen slice

**File:** `crates/app/src/features/todos/reopen.rs` (part 1 of 2)  
**Save to:** `~/code/todo/crates/app/src/features/todos/reopen.rs`

## Goal
The complete slice's shape, with no clock.

## When it passes
**◀ PR** `feat(todos): complete and reopen, owner only (TODO-8, TODO-16)` · labels `type:task,layer:http,epic:E1` · closes both task issues

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:http,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/reopen.rs`.
