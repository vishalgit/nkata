# Journey 8: long lists come in pages of ten

**File:** `end2end/tests/todos.spec.ts` (part 5 of 5)  
**Save to:** `~/code/todo/end2end/tests/todos.spec.ts`

## Goal
Append the journey.

## Check
With `bin/dev cargo leptos watch` running:
```sh
$ bin/e2e npx playwright test tests/todos.spec.ts
```

## When it passes
**◀ PR** `test(e2e): paging journey (TODO-12)` · labels `type:task,type:story,layer:e2e,epic:E2` · closes the task, TODO-12 — and epic **E2**

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,type:story,layer:e2e,epic:E2
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/end2end/tests/todos.spec.ts`.
