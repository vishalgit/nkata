# A blank title is a 422

**File:** `crates/server/tests/todos.rs` (part 2 of 13)  
**Save to:** `~/code/todo/crates/server/tests/todos.rs`

## Goal
Append the test. It passes immediately: the domain refuses, the error maps to `422`.

## When it passes
**◀ PR** `feat(todos): create_todo server function (TODO-7)` · labels `type:task,layer:http,epic:E1`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:http,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/server/tests/todos.rs`.
