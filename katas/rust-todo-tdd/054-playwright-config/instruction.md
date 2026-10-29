# end2end/playwright.config.ts

**File:** `end2end/playwright.config.ts`  
**Save to:** `~/code/todo/end2end/playwright.config.ts`

## Goal
Type the Playwright configuration: two browsers, the base URL from the environment.

## Why
- `forbidOnly` in CI: a forgotten `test.only` fails the build instead of silently skipping the rest.
- One retry in CI absorbs a rare flake; locally a failure is a failure.
- `trace: 'retain-on-failure'` keeps a step-by-step trace of every failing test for the HTML report.

## Help
- Save with `:w! ~/code/todo/end2end/playwright.config.ts`.
