# Stack and cluster

**File:** `style/layout/_primitives.scss`
**Save to:** `~/code/design-kit/style/layout/_primitives.scss`

## Goal
Two layout primitives, plus gap modifiers generated from the space scale.

## Why
- `.stack` stacks children vertically; `.cluster` lays them in a wrapping row. Between them they cover most layouts on the showcase page.
- `@each $step in map.keys($space)` writes `.stack--0 … .stack--12`. Add a step to `$space` and a modifier appears; you never maintain the list by hand.
- The selector list `.stack--#{$step}, .cluster--#{$step}` shares one rule for both.

## Check
Nothing loads it yet.

## Help
