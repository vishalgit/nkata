# Page objects — helpers.ts

**File:** `end2end/tests/pages/helpers.ts`  
**Save to:** `~/code/todo/end2end/tests/pages/helpers.ts`

## Goal
Page objects keep selectors in one place and make tests read like stories. Start with three helpers.

## Run first
```sh
$ mkdir -p end2end/tests/pages
```

## Why
- Every journey registers a fresh user (`uniqueEmail`), so tests never depend on each other's data and can run in parallel.
- `open` is the only way tests navigate: it waits for `body[data-hydrated]`.

## Help
- Save with `:w! ~/code/todo/end2end/tests/pages/helpers.ts`.
