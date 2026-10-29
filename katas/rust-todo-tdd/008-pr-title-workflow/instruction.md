# PR titles are Conventional Commits

**File:** `.github/workflows/pr-title.yml`  
**Save to:** `~/code/todo/.github/workflows/pr-title.yml`

## Goal
Type the workflow that checks every PR title from the very first PR.

## Why
- The PR title becomes the squash commit on `main`, and from Chapter 2c it decides the next version: `feat` → minor, `fix`/`perf`/`deps`/`revert` → patch, the rest → no release.
- `edited` is in the trigger list so fixing a title re-runs the check.
- Read-only permissions: the action only reads the PR.

## Help
- Save with `:w! ~/code/todo/.github/workflows/pr-title.yml`.
