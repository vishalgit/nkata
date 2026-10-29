# A render helper for component tests

**File:** `crates/app/src/ui/testing.rs`  
**Save to:** `~/code/todo/crates/app/src/ui/testing.rs`

## Goal
Render any view to an HTML string — no browser needed.

## Why
A reactive `Owner` is what signals and resources attach to; outside an app you make one yourself. `to_html()` is the same server rendering the pages use.

## Help
- Save with `:w! ~/code/todo/crates/app/src/ui/testing.rs`.
