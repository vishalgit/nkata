# Page test 3: the header shows who is logged in

**File:** `crates/server/tests/pages.rs` (part 3 of 6)  
**Save to:** `~/code/todo/crates/server/tests/pages.rs`

## Goal
Append the test: with a cookie, `/` shows the user's email **in the header**.

## Why
Assert on the element, not just the text. The page already contains `alice@example.com` without any header: Leptos serialises each resource's value (here, `me()`'s `UserView`) into the HTML so hydration need not fetch it again. A bare `contains("alice@example.com")` would pass before the header exists.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/pages.rs`.
