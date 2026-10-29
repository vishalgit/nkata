# Keep the E2E smoke test honest

**File:** `end2end/tests/smoke.spec.ts` (part 2 of 2)  
**Save to:** `~/code/todo/end2end/tests/smoke.spec.ts`

## Goal
`/` is now a redirect, so the skeleton's browser test fails — and so would this PR's pipeline. Update it in the same PR.

## Check
```sh
$ bin/e2e
```

## Help
- Save with `:w! ~/code/todo/end2end/tests/smoke.spec.ts`.
