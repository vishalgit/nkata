# The dev image — Dockerfile.dev

**File:** `Dockerfile.dev` (part 1 of 2)  
**Save to:** `~/code/todo/Dockerfile.dev`

## Goal
Type the dev image: `rust:latest`, your user as uid 1000, and the toolchain installed as that user.

## Why
- The only root step creates your user and hands it the toolchain directories. Everything after `USER` — every install, every compile — runs as uid 1000.
- `wasm32-unknown-unknown` is for the browser bundle; `x86_64-unknown-linux-musl` for the static production binary.
- `cargo binstall` downloads prebuilt cargo-leptos and bacon instead of compiling them.
- cargo-leptos downloads dart-sass, wasm-bindgen and wasm-opt into `~/.cache` on first use.

## Help
- Save with `:w! ~/code/todo/Dockerfile.dev`.
