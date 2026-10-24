# Card styles

**File:** `style/components/_card.scss`
**Save to:** `~/code/design-kit/style/components/_card.scss`

## Goal
Style the card.

## Why
- `--outlined` uses a border token; `--elevated` uses `shadow('sm')`, and the interactive hover lifts to `shadow('md')`.
- `@include transition(box-shadow, transform)` animates only those two properties.
- `overflow: hidden` clips the footer's background to the rounded corners.

## Check
Nothing loads it yet.

## Help
