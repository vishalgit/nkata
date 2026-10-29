# Story card TODO-2 · A page that proves the stack works

**File:** `backlog/stories/TODO-2.md`  
**Save to:** `~/code/todo/backlog/stories/TODO-2.md`

## Goal
Write the card for **TODO-2 · A page that proves the stack works** (E0 Delivery platform, 3 points).

A walking skeleton is the thinnest version of the whole system that runs end to end. Chapter 2 builds it in four parts: a page and a health check (2a), a production image in Kubernetes (2b), the pipeline and repository guardrails (2c), and observability (2d). Every later chapter only adds behaviour.

## Why
- The card is the issue body: `scripts/backlog story` reads `backlog/stories/TODO-2.md`.
- Each acceptance criterion names the test that will prove it. The story closes when every one exists and passes.
- Commit the card with the story's first task.

## When it passes
Create the issues from this card:

```sh
$ scripts/backlog story TODO-2 E0 enabler 3 "A page that proves the stack works"
```

## Help
- Save with `:w! ~/code/todo/backlog/stories/TODO-2.md`.
