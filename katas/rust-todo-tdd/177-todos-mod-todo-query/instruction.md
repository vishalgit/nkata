# Export TodoQuery

**File:** `crates/domain/src/todos/mod.rs` (part 10 of 10)  
**Save to:** `~/code/todo/crates/domain/src/todos/mod.rs`

## Goal
Add `TodoQuery` to the export list.

## Check
```sh
$ bin/dev cargo t-unit && bin/dev cargo lint
```
The whole business logic for todos now exists, fully tested, with no database and no web framework.

## When it passes
**◀ PR** `feat(domain): search term and the list query (TODO-11)` · labels `type:task,layer:domain,epic:E2`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:domain,epic:E2
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

**Checkpoint.** Close *Ch 04 · Workflows and query*; merge the Release PR.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/mod.rs`.
