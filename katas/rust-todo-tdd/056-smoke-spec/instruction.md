# The first browser test

**File:** `end2end/tests/smoke.spec.ts` (part 1 of 2)  
**Save to:** `~/code/todo/end2end/tests/smoke.spec.ts`

## Goal
Type the smoke test: a real browser finds the **Todo** heading.

## Check
With `cargo leptos watch` running:
```sh
$ bin/e2e                     # against the dev server: 1 test x 2 browsers
$ E2E_TARGET=k3d bin/e2e      # against Kubernetes, after bin/deploy
```

## When it passes
**◀ PR** `test(e2e): Playwright smoke test (TODO-4)` · labels `type:task,layer:e2e,epic:E0`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:e2e,epic:E0
$ gh pr ready && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/end2end/tests/smoke.spec.ts`.
