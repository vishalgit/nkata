# Keep Playwright output out of Git

**File:** `.gitignore` (part 2 of 2)  
**Save to:** `~/code/todo/.gitignore`

## Goal
Append the three Playwright folders to `.gitignore`.

## Why
`node_modules` is reinstalled by `npm ci`; results and reports are regenerated on every run.

## Help
- Save with `:w! ~/code/todo/.gitignore`.
