# Test 10: the list is paged

**File:** `crates/server/tests/todos.rs` (part 5 of 13)  
**Save to:** `~/code/todo/crates/server/tests/todos.rs`

## Goal
The remaining todo tests all create todos first: add a `create` helper at the top, then append the paging test.

## Why
It passes at once — paging was built in Chapter 5 — but it pins down the HTTP contract the UI relies on.

## When it passes
**◀ PR** `feat(todos): owner-scoped, paged list endpoint (TODO-17)` · labels `type:task,layer:http,epic:E1` · closes the TODO-17, TODO-12 and TODO-16 Chapter 7 tasks

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:http,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/server/tests/todos.rs`.
