# TDD green: implement bem()

**File:** `src/components/bem.rs` (part 2 of 2)
**Save to:** `~/code/design-kit/src/components/bem.rs`

## Goal
Replace the stub with a real implementation.

## Why
- `filter` drops empty modifiers, so callers can write `if block { "block" } else { "" }` without an `if` around the call.
- `fold` starts from the block name and pushes ` block--modifier` for each modifier.

## Check
```sh
cargo test
```
**Green**: all tests pass.

## Help
