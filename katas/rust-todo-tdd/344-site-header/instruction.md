# Green, part 2 — SiteHeader

**File:** `crates/app/src/ui/site_header.rs` (part 1 of 2)  
**Save to:** `~/code/todo/crates/app/src/ui/site_header.rs`

## Goal
The header: the user's email and a logout button, or links to log in and register.

## Why
- `Transition` keeps showing the old content while the resource refetches, so the header does not flicker after login.
- After logout the `me` resource refetches, the todos page sees nobody, and `<Redirect>` sends the browser to `/login`.

## Help
- Save with `:w! ~/code/todo/crates/app/src/ui/site_header.rs`.
