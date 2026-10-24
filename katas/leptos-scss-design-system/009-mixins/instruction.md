# Mixins

**File:** `style/abstracts/_mixins.scss`
**Save to:** `~/code/design-kit/style/abstracts/_mixins.scss`

## Goal
Write three mixins: `from($bp)` for mobile-first media queries, `focus-ring`, and `transition($props...)`.

## Why
- A **function** returns a value; a **mixin** emits declarations or whole rules.
- `@content` is the block the caller passes: `@include from('md') { … }`.
- `focus-ring` uses `&:focus-visible`, so keyboard users get an outline and mouse clicks don't.
- `$props...` is an argument list. `@include transition(color, border-color)` outputs `transition-property: color, border-color;`. Listing properties beats `transition: all`, which also animates things you didn't mean to.
- `@use 'functions' as *;` drops the namespace, so `clr('brand')` needs no prefix.

## Check
Nothing loads this file yet.

## Help
