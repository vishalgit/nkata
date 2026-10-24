# TDD red: the bem() helper

**File:** `src/components/bem.rs` (part 1 of 2)
**Save to:** `~/code/design-kit/src/components/bem.rs`

## Goal
Write the tests for `bem()` and a stub that compiles but panics.

## Why
- Components must output class names that match the SCSS exactly. `bem()` is the one place that builds them.
- **Red** first: the tests describe the behaviour, the stub makes it compile, and `cargo test` fails.
- `let _ = (block, modifiers);` silences "unused variable" warnings while the body is a `todo!()`.

## Check
Needs the next two katas (`mod.rs`, then `mod components;` in `main.rs`) before `cargo test` can see it.

## Help
