# Showcase: TextField

**File:** `src/pages/showcase.rs` (part 5 of 5)
**Save to:** `~/code/design-kit/src/pages/showcase.rs`

## Goal
Import `TextField` and add a `FieldSection` with its variants.

## Check
```sh
cargo test
```
Then review the new section in the browser in both themes.

That's the whole design system. Try these on your own:
- Add an `outline` button variant: one row in `$variants`, one enum arm. Watch the contract test fail until both are in.
- Rename `'surface-raised'` in `$colors` and read the build error.
- Add a third theme in `themes/` with its own `[data-theme='…']`.

## Help
