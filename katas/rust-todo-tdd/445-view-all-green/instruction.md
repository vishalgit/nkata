# Green — every pair explicit

**File:** `crates/domain/src/identity/policy.rs` (part 6 of 6)  
**Save to:** `~/code/todo/crates/domain/src/identity/policy.rs`

## Goal
Add `ViewAllTodos` and its two arms.

## Why
Every (role, action) pair is explicit. Add a `Role::Auditor` one day and the compiler points here until you decide what auditors may do.

## When it passes
**◀ PR** `feat(identity): authorization policy (TODO-19)` · labels `type:task,layer:domain,epic:E4`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:domain,epic:E4
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/policy.rs`.
