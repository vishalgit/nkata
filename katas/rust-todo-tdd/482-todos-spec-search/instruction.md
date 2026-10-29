# Journey 7: search narrows the list and keeps working with filters

**File:** `end2end/tests/todos.spec.ts` (part 4 of 5)  
**Save to:** `~/code/todo/end2end/tests/todos.spec.ts`

## Goal
Append the journey.

## Check
With `bin/dev cargo leptos watch` running:
```sh
$ bin/e2e npx playwright test tests/todos.spec.ts
```

## When it passes
**◀ PR** `test(e2e): search journey (TODO-11)` · labels `type:task,type:story,layer:e2e,epic:E2` · closes the task and TODO-11

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,type:story,layer:e2e,epic:E2
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/end2end/tests/todos.spec.ts`.
