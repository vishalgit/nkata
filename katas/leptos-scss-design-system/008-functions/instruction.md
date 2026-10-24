# Token accessor functions

**File:** `style/abstracts/_functions.scss`
**Save to:** `~/code/design-kit/style/abstracts/_functions.scss`

## Goal
Write `clr()`, `space()`, `radius()`, `text-size()` and `shadow()`. Each checks the name against its map and returns a `var(--…)`.

## Why
- `@use 'tokens';` loads the sibling partial under the namespace `tokens`, so you write `tokens.$colors`.
- `@use 'sass:map'` loads a built-in module: `map.has-key`, `map.get`, `map.keys`.
- The functions return `var(--color-brand)`, **not** the OKLCH value. The browser resolves the variable at runtime, which is what makes theming work.
- `@error` stops the build: `clr('brnad')` fails with the list of known names instead of shipping a broken color.
- `-lookup` starts with `-`, which makes it **private** to this module. Other files can't call it.
- Why `clr` and not `color`? `color()` is a real CSS function. A different name avoids any clash.

## Check
Nothing loads this file yet.

## Help
