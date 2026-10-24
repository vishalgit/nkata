# Element defaults

**File:** `style/base/_typography.scss`
**Save to:** `~/code/design-kit/style/base/_typography.scss`

## Goal
Set the default font and heading sizes, all from tokens.

## Why
- `base/` styles **elements** (`body`, `h1`, `code`), never classes. A class belongs to a component or a layout.
- Everything goes through accessors: `text-size('2xl')`, `clr('brand')`. Change a token and these follow.
- `--font-sans` is read with a plain `var()` because there's no map to check it against.

## Check
Nothing loads it yet.

## Help
