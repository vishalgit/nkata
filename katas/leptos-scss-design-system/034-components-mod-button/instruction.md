# Export the button

**File:** `src/components/mod.rs` (part 2 of 6)
**Save to:** `~/code/design-kit/src/components/mod.rs`

## Goal
Add `mod button;` and re-export `Button`, `ButtonSize`, `ButtonVariant`.

## Check
```sh
cargo test
```
**Expected: red.** It fails to compile: `cannot find function button_class` and `no Button in components::button`.

## Help
