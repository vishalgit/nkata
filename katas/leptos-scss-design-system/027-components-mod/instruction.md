# components/mod.rs

**File:** `src/components/mod.rs` (part 1 of 6)
**Save to:** `~/code/design-kit/src/components/mod.rs`

## Goal
Declare the `components` module with only `bem` in it.

## Why
- `mod bem;` is private. `use bem::bem;` imports the function so child modules can write `use super::bem;`.
- Components will be re-exported with `pub use` one by one as they're built.

## Check
Needs `mod components;` in `main.rs` (next kata).

## Help
