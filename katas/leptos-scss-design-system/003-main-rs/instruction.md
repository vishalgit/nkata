# Mount the app

**File:** `src/main.rs` (part 1 of 3)
**Save to:** `~/code/design-kit/src/main.rs`

## Goal
Replace the hello-world `main.rs` with one that mounts `App`.

## Why
- `mount_to_body` renders a component into `<body>`. In CSR this is the only entry point.
- `console_error_panic_hook` turns a wasm panic into a readable message in the browser console instead of `unreachable executed`.
- `mod app;` will hold the root component. `components` and `pages` join later, one at a time, so the crate compiles after every kata.

## Check
Does not compile until the next kata adds `src/app.rs`.

## Help
