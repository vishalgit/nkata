# Button styles: variant maps

**File:** `style/components/_button.scss`
**Save to:** `~/code/design-kit/style/components/_button.scss`

## Goal
Style the button from two maps, `$variants` and `$sizes`, using nested maps and `@each`.

## Why
- Each variant is one row: `(bg, fg, border, hover)`. Adding a variant means adding a row, not writing a new rule.
- `@each $name, $v in $variants { &--#{$name} { … } }` compiles to `.btn--primary`, `.btn--secondary`…
- `map.get($v, bg)` reads one column. Unquoted keys like `bg` are fine inside a nested map.
- `&:hover:not(:disabled)` stops a disabled button from reacting to hover.
- Every value is a token accessor, so the dark theme recolors the buttons for free.

## Check
Nothing loads it yet.

## Help
- The alignment spaces inside the map rows count for the kata check. Line them up as shown.
