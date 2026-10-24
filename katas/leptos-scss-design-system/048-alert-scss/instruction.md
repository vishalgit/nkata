# Alert styles

**File:** `style/components/_alert.scss`
**Save to:** `~/code/design-kit/style/components/_alert.scss`

## Goal
Style the alert.

## Why
- Here the map value is a color **family**, and `clr('#{$family}-soft')` builds the token name with interpolation inside a string.
- `grid-template-columns: 1fr auto` puts the close button in its own column; `grid-row: 1` pins it to the top whether or not there's a title.
- `border-inline-start-width: 4px` gives the colored edge on the reading-start side.

## Check
Nothing loads it yet.

## Help
