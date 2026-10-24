# Load vendors and base

**File:** `style/main.scss` (part 2 of 5)
**Save to:** `~/code/design-kit/style/main.scss`

## Goal
Add the first two `@use` lines to `main.scss`.

## Why
- `@use 'vendors';` finds `vendors/_index.scss`. Folder names, no underscores, no extensions.
- Load order is cascade order: the reset first, so base can override it.

## Check
`trunk serve` rebuilds. The heading now uses the system font at the `2xl` size. In devtools, select `<html>` and look at the `:root` rule: every token is there as a custom property.

## Help
- Build error `Undefined variable`? A partial is missing `@use '../abstracts' as *;`.
