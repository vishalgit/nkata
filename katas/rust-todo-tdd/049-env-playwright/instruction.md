# Pin the Playwright version

**File:** `.env` (part 2 of 2)  
**Save to:** `~/code/todo/.env`

**▶ Task**
```sh
scripts/backlog task TODO-4 2 e2e "Playwright service, scaffold and smoke test (2.17)" playwright
```

## Goal
Append `PLAYWRIGHT_VERSION` to `.env`: one pin for Compose and npm alike.

## Why
The browser image tag and the `@playwright/test` npm package must be the same version, or Playwright refuses to start. Use the newest release.

## Help
- Save with `:w! ~/code/todo/.env`.
