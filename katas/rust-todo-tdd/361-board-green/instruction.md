# Green, part 2 — the board

**File:** `crates/app/src/features/todos/page.rs` (part 2 of 5)  
**Save to:** `~/code/todo/crates/app/src/features/todos/page.rs`

## Goal
Replace `TodoBoard`: a create action, a list resource that refetches after it, the form, the error and the list.

## Why
- The list resource's source is `create.version()`: every finished create refetches the list.
- `SsrMode::Async` on the route means the server awaits this resource too: the first HTML already holds the todos.

## Check
Green. Open `http://localhost:3000`: you are redirected to `/login`; register, and you land on your board and can add todos — still unstyled.

## When it passes
**◀ PR** `feat(ui): the todo board with its add form (TODO-17, TODO-7)` · labels `type:task,layer:ui,epic:E1`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:ui,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/page.rs`.
