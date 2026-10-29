# Story card TODO-4 · Every push is tested and delivered

**File:** `backlog/stories/TODO-4.md`  
**Save to:** `~/code/todo/backlog/stories/TODO-4.md`

## Goal
Write the card for **TODO-4 · Every push is tested and delivered** (E0 Delivery platform, 5 points).

CI runs the same containers you use locally, so "works on my machine" and "works in CI" mean the same thing. Once the pipeline exists, `main` gets its protection, Dependabot starts, and release-please turns PR titles into versions.

## Why
- The card is the issue body: `scripts/backlog story` reads `backlog/stories/TODO-4.md`.
- Each acceptance criterion names the test that will prove it. The story closes when every one exists and passes.
- Commit the card with the story's first task.

## When it passes
Create the issues from this card:

```sh
$ scripts/backlog story TODO-4 E0 enabler 5 "Every push is tested and delivered"
```

Its Chapter 5 and Chapter 12 tasks are created when those chapters reach them.

## Help
- Save with `:w! ~/code/todo/backlog/stories/TODO-4.md`.
