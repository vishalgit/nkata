# The 1 in 7-1: the map

**File:** `style/main.scss` (part 1 of 5)
**Save to:** `~/code/design-kit/style/main.scss`

## Goal
Start `style/main.scss` with the comment that maps the seven folders. No `@use` lines yet.

## Why
- 7-1 means **7 folders + 1 entry file**. The entry file only loads folders, in a fixed order, and contains no rules.
- The order is general → specific: resets, then element defaults, then layout, components, page-specific rules, and themes last.
- `abstracts/` never appears here. It holds tokens, functions and mixins, which produce no CSS by themselves. Each partial that needs them loads them itself with `@use`.
- Modern Sass uses `@use`/`@forward`, not `@import`. `@import` is deprecated in Dart Sass and will be removed in 3.0.

## Check
```sh
trunk serve --open
```
You should see the unstyled heading. Leave `trunk serve` running: it rebuilds on every save for the rest of the set.

## Help
