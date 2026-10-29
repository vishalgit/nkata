# Properties — hundreds of titles at once

**File:** `crates/domain/src/todos/title.rs` (part 15 of 15)  
**Save to:** `~/code/todo/crates/domain/src/todos/title.rs`

## Goal
Add proptest as a dev-dependency and state two properties: `parse` never panics, and every title it returns obeys the invariants.

## Run first
```sh
$ bin/dev cargo add -p domain --dev proptest@1
```

## Why
- `".*"` is a regex strategy: proptest generates hundreds of arbitrary strings, Unicode included.
- The second property is the type's contract in two lines: 1–120 characters, nothing to trim.
- A failing input is shrunk to the smallest one that still fails and saved in `proptest-regressions/` — commit that folder.

## Check
Both pass.

## When it passes
**◀ PR** `feat(domain): typed ids and a constrained todo title (TODO-7)` · labels `type:task,layer:domain,epic:E1`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:domain,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/title.rs`.
