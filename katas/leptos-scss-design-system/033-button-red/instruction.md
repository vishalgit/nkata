# TDD red: button variants

**File:** `src/components/button.rs` (part 1 of 2)
**Save to:** `~/code/design-kit/src/components/button.rs`

## Goal
Write the variant and size enums and three tests, but not `button_class` or the component.

## Why
- `ALL` lists every variant so the showcase can loop over them, the same way the SCSS loops over `$variants`.
- `as_str()` returns the modifier name, which is also the SCSS map key.
- The third test is the **contract test**: `include_str!` reads `_button.scss` at compile time and checks that every Rust name has a `'name':` row. Rename a variant on one side only and this test fails.
- This red is a **compile error**, not a panic. In Rust that counts as red too: the test describes a function that doesn't exist yet.

## Check
Needs `mod button;` (next kata).

## Help
