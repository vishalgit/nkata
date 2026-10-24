# Site header

**File:** `style/layout/_header.scss`
**Save to:** `~/code/design-kit/style/layout/_header.scss`

## Goal
A sticky header with the brand on the left and space for the theme toggle on the right.

## Why
- This is **BEM** with Sass nesting: `&__inner` inside `.site-header` compiles to `.site-header__inner`.
- BEM: **B**lock (`site-header`), **E**lement (`__inner`, a part of the block), **M**odifier (`--sticky`, a variant of the block). Every selector is a single class, so specificity stays flat.

## Check
Nothing loads it yet.

## Help
