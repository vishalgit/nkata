# Retire the skeleton's home-page test

**File:** `crates/server/tests/skeleton.rs` (part 4 of 4)  
**Save to:** `~/code/todo/crates/server/tests/skeleton.rs`

**▶ Task**
```sh
scripts/backlog task TODO-15 8 ui "Redirect, login page and site header (8.1–8.3, 8.6)" login-page
```

## Goal
↪ Continues **TODO-15** — the redirect, the login page and the site header. The home page becomes private, so `the_home_page_renders` is now wrong: delete it, keep the two health tests.

## Why
The server functions now get a face: login and register pages, a site header, and the todo board. Pages are tested over HTTP (redirects, server-rendered markup); presentational components by rendering them to HTML in a plain unit test.

A test whose requirement changed is deleted in the same PR as the change, with a reason — not left failing or `#[ignore]`d.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/skeleton.rs`.
