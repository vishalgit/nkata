# Green — the search box, and "no matches"

**File:** `crates/app/src/features/todos/page.rs` (part 5 of 5)  
**Save to:** `~/code/todo/crates/app/src/features/todos/page.rs`

## Goal
Place the search box above the tabs, and add a first `match` arm for an empty search result.

## Why
The whole page — search results and pager included — is server-rendered, so `/?q=milk&page=2` works with JavaScript off.

## When it passes
**◀ PR** `feat(ui): search box with server-rendered results (TODO-11)` · labels `type:task,layer:ui,epic:E2`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:ui,epic:E2
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/page.rs`.
