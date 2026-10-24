# Walking-skeleton App

**File:** `src/app.rs` (part 1 of 3)
**Save to:** `~/code/design-kit/src/app.rs`

## Goal
A root component that renders a heading.

## Why
- `use leptos::prelude::*;` brings in `component`, `view!`, signals and the rest. Almost every Leptos file starts with it.
- Text in `view!` is a quoted string: `<h1>"Design Kit"</h1>`.
- `class="container"` names a class that doesn't exist yet. Layout CSS arrives in Part D; until then the page is unstyled, and that's the point of a walking skeleton: build, serve, and see it before adding design.

## Check
```sh
cargo check
```

## Help
