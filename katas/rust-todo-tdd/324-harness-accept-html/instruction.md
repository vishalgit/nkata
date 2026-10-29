# GET like a browser does

**File:** `crates/server/tests/common/mod.rs` (part 6 of 7)  
**Save to:** `~/code/todo/crates/server/tests/common/mod.rs`

## Goal
Page tests are about what a **browser** gets. Make the harness's `get` send `Accept: text/html`, as every browser navigation does.

## Why
- `leptos_axum` answers a server-side `<Redirect>` with a real `302` only when the request accepts HTML — an ordinary navigation or a plain form post. A server-function client gets a custom redirect header instead, so it can still read the function's return value.
- Without this header the redirect test in the next kata would fail for the wrong reason: a `200` with a `Location` header, which no browser ever sees.
- `/healthz` and the server functions ignore `Accept`, so every existing test stays green.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/common/mod.rs`.
