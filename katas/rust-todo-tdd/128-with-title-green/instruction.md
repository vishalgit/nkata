# Green — with_title

**File:** `crates/domain/src/todos/todo.rs` (part 10 of 10)  
**Save to:** `~/code/todo/crates/domain/src/todos/todo.rs`

## Goal
Destructure each state, change the title, rebuild the same state.

## Why
- `ActiveTodo(mut info)` destructures and binds mutably in one pattern. Only code in this module can do it: the fields are private elsewhere.
- `completed_at` is carried over untouched, so the invariant still holds.

## Check
```sh
$ bin/dev cargo t-unit && bin/dev cargo lint
```

## When it passes
**◀ PR** `feat(domain): retitle a todo in any state (TODO-9)` · labels `type:task,layer:domain,epic:E1`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:domain,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

**Checkpoint.** The pipeline now runs the unit layer on every PR. Close *Ch 03 · Domain types* and merge the Release PR.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/todo.rs`.
