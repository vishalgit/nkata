# Tokens become custom properties

**File:** `style/base/_root.scss`
**Save to:** `~/code/design-kit/style/base/_root.scss`

## Goal
Loop over every token map and write each entry as a CSS custom property on `:root`.

## Why
- This is the bridge between build time and run time. `@each $name, $value in $colors` writes `--color-brand: oklch(…)` and so on.
- `#{…}` is **interpolation**: it builds the property name (`--color-#{$name}`) and forces the value to be written out as plain text.
- After this file, `clr('brand')` → `var(--color-brand)` → a real value in the browser.
- `color-scheme: light` tells the browser to draw scrollbars and form controls in light mode. The dark theme flips it.
- `@use '../abstracts' as *;` gives this file `$colors`, `$space` etc. without a prefix.

## Check
Nothing loads it yet.

## Help
