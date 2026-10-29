# Journey 9: two users, two browsers

**File:** `end2end/tests/isolation.spec.ts`  
**Save to:** `~/code/todo/end2end/tests/isolation.spec.ts`

**▶ Task**
```sh
scripts/backlog task TODO-16 11 e2e "Isolation journey (11.10)" isolation-journey
```

## Goal
↪ Completes **TODO-16 · My todos are private** — the isolation journey. Two browser contexts are two separate cookie jars: two people.

## Check
Run everything — locally, in Kubernetes, in CI:
```sh
$ bin/e2e                                  # dev server: 10 tests x 2 browsers
$ bin/deploy && E2E_TARGET=k3d bin/e2e     # the production image in your cluster
```
The Kubernetes run matters: `LEPTOS_ENV=PROD` makes cookies `Secure`, and `bin/e2e` uses the host network so browsers accept them on `localhost`. On a failure, open the trace: `xdg-open end2end/playwright-report/index.html` — each step, the DOM at that moment, network calls and console output.

E2E users pile up in `todo_dev`; clear them with `:DB DELETE FROM users WHERE email LIKE '%@example.com'`.

## When it passes
**◀ PR** `test(e2e): users never see each other's todos (TODO-16)` · labels `type:task,type:story,layer:e2e,epic:E4` · closes the task, TODO-16 — and, with TODO-19 done, epic **E4**

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,type:story,layer:e2e,epic:E4
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

**Checkpoint.** Close *Ch 11 · E2E journeys*; merge the Release PR. Every user-facing story is done; E0 (TODO-4 and TODO-20) and E6 remain.

## Help
- Save with `:w! ~/code/todo/end2end/tests/isolation.spec.ts`.
