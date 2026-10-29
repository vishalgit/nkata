# Green — a rename form in a details element

**File:** `crates/app/src/features/todos/list.rs` (part 13 of 25)  
**Save to:** `~/code/todo/crates/app/src/features/todos/list.rs`

## Goal
After the title, add a `<details>` that opens a rename form.

## Why
- `<details>` opens and closes with no JavaScript at all: the form works before hydration too.
- `rename_id` is the rename form's own copy of the id, for the same reason as the toggle's: the form's children closure moves it.

## When it passes
**◀ PR** `feat(todos): rename slice and form (TODO-9)` · labels `type:task,layer:http,epic:E1`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:http,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/list.rs`.
