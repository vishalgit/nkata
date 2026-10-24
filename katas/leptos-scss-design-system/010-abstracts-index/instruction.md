# abstracts/_index

**File:** `style/abstracts/_index.scss`
**Save to:** `~/code/design-kit/style/abstracts/_index.scss`

## Goal
Forward the three abstracts partials through one index file.

## Why
- `@use '../abstracts'` resolves to `abstracts/_index.scss`. One line then gives a partial every token, function and mixin.
- `@forward` re-exports members without using them here. `@use` would load them only for this file.
- Because the whole folder is variables, functions and mixins, loading it any number of times outputs zero CSS. That's why `main.scss` never needs it.

## Check
Nothing loads it yet. The next part does.

## Help
