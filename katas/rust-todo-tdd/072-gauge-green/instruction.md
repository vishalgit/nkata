# Green — one line in each arm

**File:** `crates/server/src/lib.rs` (part 5 of 5)  
**Save to:** `~/code/todo/crates/server/src/lib.rs`

## Goal
Set the `app_database_up` gauge to 1 or 0 in each arm of `healthz`.

## Why
- The kubelet calls `/healthz` every five seconds, so the gauge stays fresh without a timer of its own.
- **`::metrics::`, with the leading colons.** This file declares `pub mod metrics;`, and a local module shadows the crate of the same name: plain `metrics::gauge!` looks for `gauge!` in *your* module and fails with "cannot find `gauge` in `metrics`". The leading `::` means "the external crate". (Inside `metrics.rs` itself plain `metrics::` is fine: a module is not in its own scope.)

## Check
`bin/dev cargo t-http` — green.

## When it passes
**◀ PR** `feat(metrics): request metrics and database gauge (TODO-5)` · labels `type:task,layer:observability,epic:E5`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:observability,epic:E5
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/server/src/lib.rs`.
