# vendors/_index

**File:** `style/vendors/_index.scss`
**Save to:** `~/code/design-kit/style/vendors/_index.scss`

## Goal
Load the reset from the folder index.

## Why
- In `abstracts/` the index uses `@forward`, because those files export members.
- Here it uses `@use`, because these files emit CSS. `@use` outputs a module's CSS once, however many times it's loaded.
- That's the pattern for the six folders that emit CSS: an `_index.scss` with one `@use` per partial, in the order they should appear.

## Check
Nothing loads it yet.

## Help
