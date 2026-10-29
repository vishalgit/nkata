# Journey 1: registering lands you on an empty board

**File:** `end2end/tests/auth.spec.ts` (part 1 of 3)  
**Save to:** `~/code/todo/end2end/tests/auth.spec.ts`

## Goal
Write the first journey.

## Check
With `bin/dev cargo leptos watch` running:
```sh
$ bin/e2e npx playwright test tests/auth.spec.ts
```

## When it passes
**◀ PR** `test(e2e): registration journey (TODO-14)` · labels `type:task,type:story,layer:e2e,epic:E3` · closes the task and TODO-14

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,type:story,layer:e2e,epic:E3
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/end2end/tests/auth.spec.ts`.
