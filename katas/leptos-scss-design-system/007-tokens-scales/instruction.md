# Tokens: scales

**File:** `style/abstracts/_tokens.scss` (part 2 of 2)
**Save to:** `~/code/design-kit/style/abstracts/_tokens.scss`

## Goal
Add spacing, radius, text-size, shadow and breakpoint scales, the font stacks and the motion duration.

## Why
- `$space` keys are **steps** (`4`), not pixels. Components say `space(4)`, and the scale can change without renaming anything.
- Map keys can be numbers (`$space`) or strings (`'md'`). Quote string keys so `'2xl'` and similar parse cleanly.
- A shadow value can be a comma list. Wrap it in `( … )` so the map sees one value.
- `$breakpoints` stays Sass-only. CSS can't use `var(--…)` inside a media query, so breakpoints never become custom properties.

## Check
Still nothing loads this file.

## Help
