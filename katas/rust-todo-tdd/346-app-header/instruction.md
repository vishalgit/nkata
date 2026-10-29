# Green, part 3 — place the header

**File:** `crates/app/src/app.rs` (part 4 of 7)  
**Save to:** `~/code/todo/crates/app/src/app.rs`

## Goal
Put `<SiteHeader/>` inside `<Router>`, before `<main>`.

## Why
Inside `<Router>` because the header's `<A>` links need the router.

## Check
```sh
$ bin/dev cargo t-int && bin/dev cargo t-http && bin/e2e
```

## When it passes
**◀ PR** `feat(ui): login page, redirect and site header (TODO-15)` · labels `type:task,layer:ui,epic:E3`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:ui,epic:E3
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/app/src/app.rs`.
