# An admin cannot edit someone else's todo

**File:** `crates/server/tests/admin.rs` (part 2 of 4)  
**Save to:** `~/code/todo/crates/server/tests/admin.rs`

## Goal
Append the test. It passes because the policy says so.

## When it passes
**◀ PR** `feat(todos): admins may delete any todo (TODO-19, TODO-16)` · labels `type:task,layer:http,epic:E4` · closes both task issues

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:http,epic:E4
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/server/tests/admin.rs`.
