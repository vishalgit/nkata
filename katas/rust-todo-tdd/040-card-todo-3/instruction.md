# Story card TODO-3 · Run it in Kubernetes

**File:** `backlog/stories/TODO-3.md`  
**Save to:** `~/code/todo/backlog/stories/TODO-3.md`

## Goal
Write the card for **TODO-3 · Run it in Kubernetes** (E0 Delivery platform, 5 points).

The skeleton now becomes a tiny, root-free image running in a local cluster. This comes before CI because the pipeline's deliver stage reuses `bin/deploy`.

## Why
- The card is the issue body: `scripts/backlog story` reads `backlog/stories/TODO-3.md`.
- Each acceptance criterion names the test that will prove it. The story closes when every one exists and passes.
- Commit the card with the story's first task.

## When it passes
Create the issues from this card:

```sh
$ scripts/backlog story TODO-3 E0 enabler 5 "Run it in Kubernetes"
```

## Help
- Save with `:w! ~/code/todo/backlog/stories/TODO-3.md`.
