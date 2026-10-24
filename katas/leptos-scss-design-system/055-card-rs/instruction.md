# The Card component

**File:** `src/components/card.rs`
**Save to:** `~/code/design-kit/src/components/card.rs`

## Goal
Write the `Card` component with its tests.

## Why
- `#[slot] pub struct CardFooter` declares a **slot**: a named child the caller passes as `<CardFooter slot>…</CardFooter>`.
- The prop is named `card_footer`, the snake_case of the slot's type. That's how Leptos matches them.
- `tabindex=interactive.then_some("0")` makes only interactive cards reachable with Tab, so `focus-ring` applies to them.

## Check
Needs the export in `mod.rs` (next kata).

## Help
