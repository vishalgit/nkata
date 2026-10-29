# The check belongs in CI

**File:** `.github/workflows/pipeline.yml` (part 5 of 7)  
**Save to:** `~/code/todo/.github/workflows/pipeline.yml`

## Goal
In the `test` job, right after `actions/checkout@v4`, run `bin/check-overlays`.

## Why
GitHub's Ubuntu runners include `kubectl`; the check needs no cluster, only `kubectl kustomize`.

## Help
- Save with `:w! ~/code/todo/.github/workflows/pipeline.yml`.
