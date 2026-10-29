# Tests 11 and 12: search over HTTP, and its limit

**File:** `crates/server/tests/todos.rs` (part 6 of 13)  
**Save to:** `~/code/todo/crates/server/tests/todos.rs`

**▶ Task**
```sh
TASK_ONLY=1 scripts/backlog task TODO-10 7 http "Filter parameter on the list endpoint (7.9, 7.11)"
scripts/backlog task TODO-11 7 http "Search over HTTP (7.11)" search-http
```

## Goal
↪ Continues **TODO-10 and TODO-11** — search and the filter over HTTP. Append two tests.

## Why
Both pass at once — the store and the domain do the work; the HTTP tests prove the wiring (`SearchError` → `Validation` → `422`). A page past the end (`page=99`) returns empty `items` with the true `total` — worth a documentation test of your own.

## When it passes
**◀ PR** `test(todos): search and filter over HTTP (TODO-11, TODO-10)` · labels `type:task,layer:http,epic:E2`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:http,epic:E2
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

**Try it by hand**, with `bin/dev cargo leptos watch` running:
```sh
$ curl -si -X POST localhost:3000/api/auth/register -d 'email=me@example.com&password=correct horse battery'
$ curl -si -X POST localhost:3000/api/todos/create -d 'title=Try curl' -H 'Cookie: todo_session=PASTE_TOKEN'
```
Argon2 deliberately burns CPU; on a busy server it belongs in `tokio::task::spawn_blocking` — a Chapter 12 hardening item.

**Checkpoint.**
```sh
$ bin/dev cargo t-unit && bin/dev cargo t-int && bin/dev cargo t-http
$ bin/deploy
$ curl -si -X POST localhost:8080/api/auth/register -d 'email=k8s@example.com&password=correct horse battery'
```
In Kubernetes the `Set-Cookie` carries `Secure`, because the image runs with `LEPTOS_ENV=PROD`. Close *Ch 07 · Server functions*; merge the Release PR.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/todos.rs`.
