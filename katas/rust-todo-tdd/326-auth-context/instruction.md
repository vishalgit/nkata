# Green, part 1 — who is logged in, for every component

**File:** `crates/app/src/features/auth/context.rs` (part 1 of 4)  
**Save to:** `~/code/todo/crates/app/src/features/auth/context.rs`

## Goal
Type the auth context: a resource that asks `me()` once per page, shared through context.

## Why
- A `Resource` runs an async function and holds its result; on the server, Leptos awaits it while rendering and sends the value to the browser with the HTML, so hydration does not call `me()` again.
- `Copy`: Leptos signals and resources are cheap handles, so the whole struct can be copied into closures.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/context.rs`.
