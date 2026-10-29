# Green, part 2 — the page joins its slice

**File:** `crates/app/src/features/auth/login.rs` (part 3 of 3)  
**Save to:** `~/code/todo/crates/app/src/features/auth/login.rs`

## Goal
Append `LoginPage` to `login.rs`: the endpoint and its page live together.

## Why
- **Vertical slice**: everything about logging in — server function, shell, page — is one file.
- `ActionForm` renders a real `<form action="/api/auth/login">`. Once hydrated, it submits through the action instead, without a page load.
- The `Effect` runs in the browser only: when the action succeeds, go to `/`.
- The `use` lines sit below the server function on purpose: they belong to the page half of the slice.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/login.rs`.
