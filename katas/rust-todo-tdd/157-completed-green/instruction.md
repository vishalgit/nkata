# Green — Filter::completed

**File:** `crates/domain/src/todos/filter.rs` (part 6 of 6)  
**Save to:** `~/code/todo/crates/domain/src/todos/filter.rs`

## Goal
Add `completed`.

## Why
`Option<bool>` maps straight onto a nullable SQL parameter in Chapter 5: `NULL` switches the condition off.

## When it passes
**◀ PR** `feat(domain): todo filter (TODO-10)` · labels `type:task,layer:domain,epic:E2`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:domain,epic:E2
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/filter.rs`.
