# TodosPage gains search

**File:** `end2end/tests/pages/todos-page.ts` (part 4 of 5)  
**Save to:** `~/code/todo/end2end/tests/pages/todos-page.ts`

**▶ Task**
```sh
scripts/backlog task TODO-11 11 e2e "Search journey (11.9)" search-journey
```

## Goal
↪ Completes **TODO-11 · Search my todos** — the search journey. Add `search`.

## Why
The search box is a real `GET` form: submitting it loads a new page, so `search` waits for the URL and for hydration again.

## Help
- Save with `:w! ~/code/todo/end2end/tests/pages/todos-page.ts`.
