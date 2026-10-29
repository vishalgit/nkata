# Journey 3: a wrong password shows a generic error

**File:** `end2end/tests/auth.spec.ts` (part 3 of 3)  
**Save to:** `~/code/todo/end2end/tests/auth.spec.ts`

## Goal
Append the journey.

## Check
With `bin/dev cargo leptos watch` running:
```sh
$ bin/e2e npx playwright test tests/auth.spec.ts
```

## When it passes
**◀ PR** `test(e2e): login and logout journeys (TODO-15)` · labels `type:task,type:story,layer:e2e,epic:E3` · closes the task, TODO-15 — and epic **E3**

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,type:story,layer:e2e,epic:E3
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/end2end/tests/auth.spec.ts`.
