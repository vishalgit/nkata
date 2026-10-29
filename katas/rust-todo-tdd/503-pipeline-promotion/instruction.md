# The pipeline — dev, test, your approval, prod

**File:** `.github/workflows/pipeline.yml` (part 7 of 7)  
**Save to:** `~/code/todo/.github/workflows/pipeline.yml`

## Goal
Append four jobs after `release`: deploy to dev, deploy to test and run E2E there, wait for your approval, deploy to prod.

## Why
- Your GitHub Pro plan gives private repositories **environments**, so each deploy job names one and appears under Deployments with its URL and history. Required reviewers on a private repo would need Enterprise, so the approval is an issue instead.
- Actions sets `GITHUB_REPOSITORY_OWNER` for every job, and the job's `GITHUB_TOKEN` can read the image this repo's workflow published.
- Each `concurrency` group allows one deploy at a time per environment; a newer run waits its turn.
- `approve-prod` also waits for `release`, only to read its tag for the issue title; `!cancelled()` makes a failed release harmless.

**The approval gate.** When test is green, `approve-prod` opens an issue titled "Promote `v0.9.0` to prod" for a release (or the commit hash for an ordinary merge) and polls it on the `gate` runner, costing no Actions minutes. Look at `http://test.todo.localhost:8080` and the dashboard, then comment `approve` (or `lgtm`, `yes`) and `deploy-prod` starts; `deny` (or `no`) fails the run and prod keeps its version. Either way the issue closes and records who decided and when, so `is:issue in:title "to prod"` is your release log. A newer merge waits behind an open approval: deny the older one to move straight to the newer.

**Rollback.** `kubectl -n todo-prod rollout undo deployment/todo` returns to the previous version at once. For a specific older commit, `bin/promote prod <sha>` by hand, logged in as above.

**Declaring 1.0.0.** This is the last Chapter 12 task, so put `Release-As: 1.0.0` on its own line in this PR's body. The body becomes the squash commit's body, and the next Release PR proposes `1.0.0`.

## When it passes
**◀ PR** `feat(ci): promote through dev and test to prod with approval (TODO-20)` · labels `type:task,type:enabler,layer:infra,epic:E0` · body ends with `Release-As: 1.0.0` · closes the task, TODO-20 — and epic **E0**

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,type:enabler,layer:infra,epic:E0
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

**Acceptance test.** After the merge, watch Actions: `test` → `deliver` → `release` → `deploy-dev` → `deploy-test` (with E2E) → `approve-prod` waiting. Comment `approve`; `deploy-prod` ends with `ok <- prod runs todo:<sha>`, and `http://todo.localhost:8080` serves the change. Merge another small PR and comment `deny`: the run fails and prod stays where it was. That is every criterion on TODO-20's card.

**Releasing 1.0.0.** Close *Ch 12 · Hardening* once its tasks are done, read the Release PR (`chore(main): release 1.0.0`) and merge it. Its pipeline run tags the tested image `todo:v1.0.0`, creates the GitHub Release and its changelog, and opens the issue "Promote v1.0.0 to prod". Approve it.

**Backlog · Epic E6 Production readiness.** Each item is a story waiting to be written, numbered from TODO-21; nothing exists on GitHub until you pick one. The first time: `scripts/backlog epic E6 "Production readiness" "Hardening before real users"`. Then write its card, `scripts/backlog story TODO-<n> E6 enabler <points> "<title>"`, and start each task with `scripts/backlog task TODO-<n> 12 <layer> "<title>" <slug>` — like every other story.

- [ ] Run Argon2 on a blocking thread: wrap `register_user(..)` and `authenticate(..)` in `tokio::task::spawn_blocking`.
- [ ] Rate-limit `/api/auth/login` and `/api/auth/register` per client (`tower_governor`); the 11th attempt in a minute gets `429`.
- [ ] CSRF defence in depth: an `Origin` header check on top of `SameSite=Lax` and POST-only server functions.
- [ ] Delete expired sessions on a schedule — a `CronJob` running `DELETE FROM sessions WHERE expires_at < now()`.
- [ ] Security headers (`Content-Security-Policy`, `X-Content-Type-Options`, `Referrer-Policy`) with `tower-http`'s `SetResponseHeaderLayer`.
- [ ] Request tracing with `tower-http`'s `TraceLayer` and a request id on every log line.
- [ ] Graceful shutdown: `axum::serve(..).with_graceful_shutdown(signal)`, so rolling updates drop no requests.
- [ ] A `NetworkPolicy` so only the app pods can reach Postgres.
- [ ] TLS at the ingress (Traefik with cert-manager) for anything beyond your laptop.
- [ ] Database backups: a `CronJob` running `pg_dump` to a volume, and a restore drill.

**A faster pipeline.** Cache Docker layers between runs with `docker/build-push-action` and `cache-from: type=gha`; split dependency compilation from your own code in the production `Dockerfile` with `cargo-chef`.

**Exercises — keep the TDD habit.**
1. **Due dates.** A `DueDate` constrained type (not in the past at creation), an `Overdue` derived state, and a column. Start with one domain test.
2. **Sharing.** Share a list read-only with another user. Add `Action::ViewTodo { owner, shared_with }` to the policy table first.
3. **Keyset pagination.** Replace `OFFSET` with keyset pagination on `(created_at, id)`; the HTTP tests are the specification.
4. **Optimistic UI.** Use `action.input()` to show a pending todo before the server answers; cover it with an E2E journey.
5. **Mutation testing.** `bin/dev cargo install --locked cargo-mutants && bin/dev cargo mutants -p domain`; add tests until no mutant survives.
6. **Coverage.** `cargo llvm-cov` for the domain and app crates; aim for 100% in the domain.
7. **Leptos 0.9.** When it is released, upgrade on a branch; the test layers and the pipeline tell you exactly what broke.
8. **Per-environment monitoring.** A `namespace` variable on the dashboard and alert rules grouped `by (namespace)`, so dev failing never looks like prod failing. Prove each alert with the Chapter 2 drill in `todo-dev`.

**Final checkpoint.**
```sh
$ bin/dev cargo t-unit && bin/dev cargo t-int && bin/dev cargo t-http && bin/e2e
$ gh release view                       # v1.0.0, its notes, its date
$ curl -s http://todo.localhost:8080/healthz
```
Every epic but E6 is closed, `v1.0.0` runs in prod, and every future change takes the same road: issue, branch, red, green, PR, dev, test, your approval.

## Help
- Save with `:w! ~/code/todo/.github/workflows/pipeline.yml`.
