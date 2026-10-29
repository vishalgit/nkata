# Green — a toggle form, and the old tests learn about context

**File:** `crates/app/src/features/todos/list.rs` (part 11 of 25)  
**Save to:** `~/code/todo/crates/app/src/features/todos/list.rs`

## Goal
Give `TodoItem` a Done or Undo form, then switch the Chapter 8 item tests to `render_item` (they would panic: no `TodoActions` in context).

## Why
- Every click is a real form post with a hidden `id`, so it works even before the WASM bundle loads; once hydrated, `ActionForm` submits through the action.
- `ActionForm`'s children are a closure, and a closure takes what it uses **by move**. Each form therefore gets its own copy of the id (`let id = id.clone();` inside the branch), so the original stays available for the forms still to come.
- The list test provides the actions inside its render closure, like `render_item` does.

## Check
`bin/dev cargo t-int` — green.

## When it passes
**◀ PR** `feat(ui): Done and Undo on every item (TODO-8)` · labels `type:task,layer:ui,epic:E1`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:ui,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/list.rs`.
