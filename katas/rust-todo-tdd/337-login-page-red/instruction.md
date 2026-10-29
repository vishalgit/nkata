# Page test 2: the login page renders a working form

**File:** `crates/server/tests/pages.rs` (part 2 of 6)  
**Save to:** `~/code/todo/crates/server/tests/pages.rs`

## Goal
Append the test: `/login` has the two fields and a form that posts to the login server function.

## Why
`action="/api/auth/login"` means the form works even before the WASM loads: a plain HTML form post to the server function.

## Check
Red: `404`.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/pages.rs`.
