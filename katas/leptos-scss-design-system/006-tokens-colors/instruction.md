# Tokens: color

**File:** `style/abstracts/_tokens.scss` (part 1 of 2)
**Save to:** `~/code/design-kit/style/abstracts/_tokens.scss`

## Goal
Define the color tokens as two Sass maps: `$colors` and `$dark-colors`.

## Why
- A **token** is a named design decision: `'danger'` instead of `oklch(58% 0.21 25)`. Components use names, never raw values.
- Names are **semantic** (`surface`, `text-muted`, `danger-soft`), not visual (`gray-100`). That's what lets dark mode swap values under the same names.
- Each family has three steps: the base color, `-strong` for text on a soft background, `-soft` for backgrounds.
- OKLCH: lightness in %, chroma (colorfulness), hue in degrees. Same L looks equally bright across hues, so `success` and `danger` sit at matching weights.
- The `_` prefix makes this a **partial**: Sass never compiles it alone, only when another file loads it.

## Check
Nothing loads this file yet.

## Help
