# Story card TODO-10 · Filter by state

**File:** `backlog/stories/TODO-10.md`  
**Save to:** `~/code/todo/backlog/stories/TODO-10.md`

## Goal
Write the card for **TODO-10 · Filter by state** (E2 Find my todos, 3 points).

## Why
- The card is the issue body: `scripts/backlog story` reads `backlog/stories/TODO-10.md`.
- Each acceptance criterion names the test that will prove it. The story closes when every one exists and passes.
- Commit the card with the story's first task.

## When it passes
Create the issues from this card:

```sh
$ scripts/backlog epic E2 "Find my todos" "Filter, search and page through long lists"
$ scripts/backlog story TODO-10 E2 story 3 "Filter by state"
```

A new epic starts here, so its issue comes first.

## Help
- Save with `:w! ~/code/todo/backlog/stories/TODO-10.md`.
