# Piece 2 — effects.rs

**File:** `crates/app/src/effects.rs` (part 1 of 4)  
**Save to:** `~/code/todo/crates/app/src/effects.rs`

## Goal
Type the effects module: the only door to the outside world for server functions.

## Why
- `state()` reads the `AppState` that `leptos_routes_with_context` provided for this request.
- `fail` sets the HTTP status from the error. Without it every error would be a `500`.
- `start_session` generates a token, stores only its hash, and sets the cookie — `Secure` in production (`LEPTOS_ENV=PROD` in the image), not on your laptop.
- Keeping every effect here means the slices read as gather → decide → persist, with no plumbing.

## Help
- Save with `:w! ~/code/todo/crates/app/src/effects.rs`.
