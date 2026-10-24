# The dark theme

**File:** `style/themes/_dark.scss`
**Save to:** `~/code/design-kit/style/themes/_dark.scss`

## Goal
Write the dark theme: the same `--color-*` names with new values, under `[data-theme='dark']`.

## Why
- A theme changes **values**, never selectors. No component file mentions dark mode.
- The first `@each` checks the build: every key in `$dark-colors` must also be in `$colors`. Misspell `'surfce'` and the build fails.
- `$_` is a name for a loop value you don't use.
- Custom properties are inherited. Put `data-theme="dark"` on any element and everything inside it repaints.

## Check
Nothing loads it yet.

## Help
