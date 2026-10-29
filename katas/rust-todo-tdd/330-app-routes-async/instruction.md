# Green, part 3 — route / to the todos page, asynchronously

**File:** `crates/app/src/app.rs` (part 2 of 7)  
**Save to:** `~/code/todo/crates/app/src/app.rs`

## Goal
Delete `HomePage`, provide the auth context, and route `/` to `TodosPage` with `SsrMode::Async`.

## Why
By default Leptos streams HTML and sends the headers before `me()` answers, so a redirect could only happen in the browser. `SsrMode::Async` waits for the page's resources first, so the server can still answer `302 Location: /login`.

## Check
`bin/dev cargo t-http` — green.

## Help
- Save with `:w! ~/code/todo/crates/app/src/app.rs`.
