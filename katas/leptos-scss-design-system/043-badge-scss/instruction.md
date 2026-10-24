# Badge styles

**File:** `style/components/_badge.scss`
**Save to:** `~/code/design-kit/style/components/_badge.scss`

## Goal
Style the badge.

## Why
- Each tone maps to a **list** of two token names: `('success-soft', 'success-strong')`.
- `list.nth($pair, 1)` and `list.nth($pair, 2)` read them. Sass lists start at 1.
- The values in the map are token **names**, and `clr()` resolves them. A typo still fails the build.

## Check
Nothing loads it yet.

## Help
