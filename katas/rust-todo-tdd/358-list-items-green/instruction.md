# Green — TodoList

**File:** `crates/app/src/features/todos/list.rs` (part 9 of 25)  
**Save to:** `~/code/todo/crates/app/src/features/todos/list.rs`

## Goal
An empty list shows the hint; otherwise a `<ul>` with one `TodoItem` each.

## Why
`collect_view()` turns an iterator of views into one view. `<TodoItem todo/>` is shorthand for `todo=todo`.

## When it passes
**◀ PR** `feat(ui): todo item and list components (TODO-17)` · labels `type:task,layer:ui,epic:E1`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:ui,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/list.rs`.
