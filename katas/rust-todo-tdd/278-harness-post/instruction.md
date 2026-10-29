# The harness learns to POST, read JSON and cookies

**File:** `crates/server/tests/common/mod.rs` (part 4 of 7)  
**Save to:** `~/code/todo/crates/server/tests/common/mod.rs`

**▶ Task**
```sh
scripts/backlog task TODO-14 7 http "Register endpoint (7.1, 7.3–7.4)" register-endpoint
```

## Goal
A Leptos `#[server]` function is an Axum endpoint you can also call like an async function. Give the harness what testing one needs: a form `post`, a `json` reader, and the session cookie from `Set-Cookie`.

## Run first
```sh
$ bin/dev cargo add -p server --dev serde@1 serde_json@1 serde_urlencoded@0.7
```

| Endpoint | Server function | Slice file | Task |
| --- | --- | --- | --- |
| `POST /api/auth/register` | `register(email, password)` | `features/auth/register.rs` | TODO-14 |
| `POST /api/auth/me` | `me()` | `features/auth/me.rs` | TODO-15 |
| `POST /api/auth/login` | `login(email, password)` | `features/auth/login.rs` | TODO-15 |
| `POST /api/auth/logout` | `logout()` | `features/auth/logout.rs` | TODO-15 |
| `POST /api/todos/create` | `create_todo(title)` | `features/todos/create.rs` | TODO-7 |
| `POST /api/todos/list` | `list_todos(filter, q, page)` | `features/todos/list.rs` | TODO-17, 12, 16, 10, 11 |
Server functions accept form-encoded bodies and answer with JSON. Errors are your own `AppError`, serialised as JSON with a matching HTTP status.

## Why
- `session_cookie` keeps only `todo_session=<token>`, the part a browser sends back.
- `PASSWORD` is one valid password every test can share.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/common/mod.rs`.
