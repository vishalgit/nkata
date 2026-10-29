# The board reads the query from the URL

**File:** `crates/app/src/features/todos/page.rs` (part 4 of 5)  
**Save to:** `~/code/todo/crates/app/src/features/todos/page.rs`

## Goal
The board now reads all three URL parameters, passes them to `list_todos`, and renders the tabs and the pager.

## Why
- The resource's source now includes the URL parameters: clicking a tab or a page link changes the URL, which changes the source, which refetches.
- `get_untracked()` inside the render closure: reading them tracked would re-render the pager on its own; the resource already re-renders it.

## Check
```sh
$ bin/dev cargo t-int && bin/dev cargo t-http
```

## When it passes
**◀ PR** `feat(ui): filter tabs and pager (TODO-12, TODO-10)` · labels `type:task,layer:ui,epic:E2` · closes both task issues

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:ui,epic:E2
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/page.rs`.
