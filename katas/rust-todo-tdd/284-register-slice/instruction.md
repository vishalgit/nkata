# Piece 4 — the register slice

**File:** `crates/app/src/features/auth/register.rs` (part 1 of 2)  
**Save to:** `~/code/todo/crates/app/src/features/auth/register.rs`

## Goal
Every slice has this shape: a public `#[server]` function delegating to a private, server-only `shell::handle`, then `.map_err(effects::fail)`.

## Why
- The shell is the sandwich from Chapter 4, written out: gather (state, id, clock) → decide (`register_user`) → persist and respond.
- `endpoint = "auth/register"` plus the default `/api` prefix gives `POST /api/auth/register`.
- `?` converts `RegisterError` and `AuthStoreError` into `AppError` through the `From` impls.

## Check
`bin/dev cargo t-http` — green.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/auth/register.rs`.
