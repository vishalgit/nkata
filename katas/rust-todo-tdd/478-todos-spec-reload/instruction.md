# Journey 5: todos survive a reload

**File:** `end2end/tests/todos.spec.ts` (part 2 of 5)  
**Save to:** `~/code/todo/end2end/tests/todos.spec.ts`

## Goal
Append the journey.

## Check
With `bin/dev cargo leptos watch` running:
```sh
$ bin/e2e npx playwright test tests/todos.spec.ts
```

## When it passes
**◀ PR** `test(e2e): todos survive a reload (TODO-17)` · labels `type:task,type:story,layer:e2e,epic:E1` · closes the task and TODO-17 — and, with TODO-18 already done, epic **E1**

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,type:story,layer:e2e,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/end2end/tests/todos.spec.ts`.
