# Container

**File:** `style/layout/_container.scss`
**Save to:** `~/code/design-kit/style/layout/_container.scss`

## Goal
A centered, max-width container with padding that grows from the `md` breakpoint.

## Why
- `@include from('md') { … }` is the mixin you wrote. It compiles to `@media (min-width: 48rem)`.
- Mobile first: the base rule is the small screen; the media query adds to it.
- `margin-inline` and `padding-inline` are logical properties: they follow the text direction, so the layout also works right-to-left.

## Check
Nothing loads it yet.

## Help
