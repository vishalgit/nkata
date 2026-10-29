# Continuous tests — bacon.toml

**File:** `bacon.toml` (part 1 of 3)  
**Save to:** `~/code/todo/bacon.toml`

## Goal
Type `bacon.toml` with one job for the HTTP layer.

## Why
- Each job uses its own target directory, so it never waits on cargo-leptos for the build lock.
- One job now; the others join with their test layers. In the tests window: `bin/dev bacon http`.

## When it passes
**◀ PR** `feat: health check that reflects the database (TODO-2)` · labels `type:task,type:enabler,layer:http,epic:E0` · closes the task and TODO-2

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,type:enabler,layer:http,epic:E0
$ gh pr ready && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/bacon.toml`.
