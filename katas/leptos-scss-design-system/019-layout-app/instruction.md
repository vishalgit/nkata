# The app shell

**File:** `style/layout/_app.scss`
**Save to:** `~/code/design-kit/style/layout/_app.scss`

## Goal
Style `.app`, the wrapper that will carry `data-theme`.

## Why
- The theme attribute goes on a `<div class="app">` that Leptos controls, not on `<html>`, which a CSR Leptos app doesn't render.
- Because `.app` sets its own background and text color from tokens, switching the theme repaints the whole page.
- `layout/` holds page structure: the shell, containers, header, spacing primitives.

## Check
Nothing loads it yet.

## Help
