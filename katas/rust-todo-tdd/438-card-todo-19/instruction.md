# Story card TODO-19 · Admins moderate

**File:** `backlog/stories/TODO-19.md`  
**Save to:** `~/code/todo/backlog/stories/TODO-19.md`

## Goal
Write the card for **TODO-19 · Admins moderate** (E4 Privacy and roles, 5 points).

Authentication says *who you are*; authorization says *what you may do*. Today "only the owner may touch a todo" is an `if` inside `access.rs`. This chapter moves every such rule into one pure policy function, adds an **admin** role, and builds an admin page — with the Chapter 9 tests guarding the refactor.

| Action | User, own todo | User, other's todo | Admin, own todo | Admin, other's todo |
| --- | --- | --- | --- | --- |
| Edit (complete, reopen, rename) | allowed | denied | allowed | denied |
| Delete | allowed | denied | allowed | allowed (moderation) |
| View all todos | denied | denied | allowed | allowed |

Admins moderate (view and delete) but never put words in other people's mouths (edit). Each row becomes one test.

## Why
- The card is the issue body: `scripts/backlog story` reads `backlog/stories/TODO-19.md`.
- Each acceptance criterion names the test that will prove it. The story closes when every one exists and passes.
- Commit the card with the story's first task.

## When it passes
Create the issues from this card:

```sh
$ scripts/backlog story TODO-19 E4 story 5 "Admins moderate"
```

## Help
- Save with `:w! ~/code/todo/backlog/stories/TODO-19.md`.
