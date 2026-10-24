# Render the showcase

**File:** `src/app.rs` (part 3 of 3)
**Save to:** `~/code/design-kit/src/app.rs`

## Goal
Swap the plain button for `<Button>` and render `<Showcase />` in `<main>`.

## Why
- The toggle is now built from the design system it toggles: `variant=ButtonVariant::Secondary size=ButtonSize::Sm`.
- The button label is a closure, so it reads "Dark theme" or "Light theme" from the signal.

## Check
The browser shows the swatches and the button matrix. Toggle the theme: chips and buttons recolor, and no component code changed.

## Help
