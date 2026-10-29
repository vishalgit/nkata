# The HTTP test harness

**File:** `crates/server/tests/common/mod.rs` (part 1 of 7)  
**Save to:** `~/code/todo/crates/server/tests/common/mod.rs`

**▶ Task**
```sh
scripts/backlog task TODO-2 2 http "App, frontend and server crates from the first HTTP test (2.4–2.8)" first-page
```

## Goal
Start from the outside: type the harness every HTTP test file shares. It grows one helper at a time.

## Why
- `tower::ServiceExt::oneshot` sends one request through the real Axum router, in memory: no port, no network, but every route, middleware and status code is real.
- `without_database` builds a pool that can never connect (port 1), with a one-second timeout. The router still starts, because the pool is lazy.
- `get` takes an optional cookie from the start, so its signature never changes when login arrives in Chapter 7.
- `#![allow(dead_code)]`: each test file compiles this module separately and uses only some helpers.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/common/mod.rs`.
