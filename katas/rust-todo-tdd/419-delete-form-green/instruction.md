# Green — the delete button

**File:** `crates/app/src/features/todos/list.rs` (part 15 of 25)  
**Save to:** `~/code/todo/crates/app/src/features/todos/list.rs`

## Goal
After `</details>`, inside `todo-item__actions`, add the delete form.

## Why
The last use of `id` takes it by value — no `.clone()` needed.

## When it passes
**◀ PR** `feat(todos): delete slice and button (TODO-13)` · labels `type:task,layer:http,epic:E1`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:http,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/list.rs`.
