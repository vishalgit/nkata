# Showcase: tokens and buttons

**File:** `src/pages/showcase.rs` (part 1 of 5)
**Save to:** `~/code/design-kit/src/pages/showcase.rs`

## Goal
Build the review page: a swatch for every color token, and a variant × size matrix of buttons.

## Why
- `ShowcaseSection` is a private helper component, so each section has the same header markup.
- `COLOR_TOKENS` mirrors `$colors`. The chips read `var(--color-{name})` directly, so they show whatever the current theme says.
- The test at the bottom checks every name in `COLOR_TOKENS` exists in `_tokens.scss`.
- `ButtonVariant::ALL.into_iter().map(…).collect_view()` renders a list. The nested loop gives one row per variant, one column per size.
- The `loading` signal shows the state demo: click **Click to load** to see the spinner and the disabled state, **Reset** to clear it.

## Check
Needs `pages/mod.rs` and `mod pages;` (next two katas).

## Help
