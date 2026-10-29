# The pipeline runs the unit layer

**File:** `.github/workflows/pipeline.yml` (part 3 of 7)  
**Save to:** `~/code/todo/.github/workflows/pipeline.yml`

## Goal
Add a **Unit tests** step before the HTTP tests.

## Why
Fastest layer first: a broken business rule fails the run in seconds, before the slower layers start.

## Help
- Save with `:w! ~/code/todo/.github/workflows/pipeline.yml`.
