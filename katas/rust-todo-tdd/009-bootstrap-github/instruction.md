# The one-time GitHub bootstrap

**File:** `scripts/bootstrap-github.sh`  
**Save to:** `~/code/todo/scripts/bootstrap-github.sh`

## Goal
Type the script that sets the repository's merge rules, creates the fixed labels, and makes the empty **Todo book** project with its fields.

## Run first
```sh
$ touch backlog/stories/.gitkeep
```

## Why
- Squash merges only, with the PR title as the commit title: `main` gets one Conventional Commit per task.
- `--force` on `gh label create` updates a label that already exists, so the script can be re-run.
- Epic labels, milestones and issues are **not** created here. `scripts/backlog` makes each one when the guide reaches it.
- `backlog/stories/` holds one card per story. When a kata shows a story card, save it there before `scripts/backlog story` runs: it becomes the issue body.

## Help
- Save with `:w! ~/code/todo/scripts/bootstrap-github.sh`.
