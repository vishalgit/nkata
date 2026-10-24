# Load layout and themes

**File:** `style/main.scss` (part 3 of 5)
**Save to:** `~/code/design-kit/style/main.scss`

## Goal
Add `layout` and `themes` to `main.scss`.

## Why
- `themes` goes **last**. `[data-theme='dark']` and `:root` have the same specificity, so source order decides any tie.
- `components` and `pages` will go between `layout` and `themes`.

## Check
`.container` now pads the page. Nothing carries `data-theme` yet; the next kata adds the `.app` wrapper and the toggle.

## Help
