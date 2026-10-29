# The item's new elements

**File:** `style/components/_todo-item.scss` (part 2 of 2)  
**Save to:** `~/code/todo/style/components/_todo-item.scss`

## Goal
Inside `.todo-item { … }`, after `&__title`, style the actions, the open rename panel and the rename form.

## Why
`&__edit[open]` styles the `<details>` only while it is open: it takes a full row, so the rename form has room.

## Check
```sh
$ bin/dev cargo t-unit && bin/dev cargo t-int && bin/dev cargo t-http && bin/e2e
```
Add, complete, undo, rename, delete, search, switch filters and page — locally or on `localhost:8080`.

## When it passes
**◀ PR** `feat(ui): styles for search, pager, tabs and item actions (TODO-18)` · labels `type:task,layer:ui,epic:E1`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:ui,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

**Checkpoint.** Close *Ch 09 · Slices and list UI*; merge the Release PR.

## Help
- Save with `:w! ~/code/todo/style/components/_todo-item.scss`.
