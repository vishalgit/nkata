# Journey 4: the life of a todo

**File:** `end2end/tests/todos.spec.ts` (part 1 of 5)  
**Save to:** `~/code/todo/end2end/tests/todos.spec.ts`

## Goal
Every test in this file starts as a fresh, registered user. The first journey reads like the story it proves.

## Check
With `bin/dev cargo leptos watch` running:
```sh
$ bin/e2e npx playwright test tests/todos.spec.ts
```

## When it passes
**◀ PR** `test(e2e): the life of a todo (TODO-7, TODO-8, TODO-9, TODO-13)` · labels `type:task,type:story,layer:e2e,epic:E1` · body closes the four task issues and the four stories

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,type:story,layer:e2e,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/end2end/tests/todos.spec.ts`.
