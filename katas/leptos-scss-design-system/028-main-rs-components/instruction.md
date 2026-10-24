# Register components

**File:** `src/main.rs` (part 2 of 3)
**Save to:** `~/code/design-kit/src/main.rs`

## Goal
Add `mod components;` to `main.rs`.

## Check
```sh
cargo test
```
**Expected: red.** Three tests fail with `not yet implemented: make the tests pass`. That failure is what you want to see before writing the code.

## Help
- Two warnings are expected: `unused import: bem::bem` and `function bem is never used`. They go away once the Button calls `bem()`.
