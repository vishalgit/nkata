**Epic:** E0 Delivery platform · **Type:** enabler · **Points:** 5

*As the* developer *I want* CI/CD from the first commit *so that* a broken change never reaches the cluster.

### Acceptance criteria

- [ ] A real browser passes against the dev server → `the app is up` (later `a visitor lands on the login page`)
- [ ] A push runs the checks and every test layer in the same containers as your laptop (2.18)
- [ ] Each push deploys to a throwaway k3d cluster and smoke-tests it; `main` publishes the image (2.18)

### Tasks

- [ ] Ch 2 · Playwright service, scaffold and smoke test (2.17)
- [ ] Ch 2 · `pipeline.yml` (2.18)
- [ ] Ch 5 · Migrations and offline query data in the pipeline (5.15)
- [ ] Ch 12 · Self-hosted runners on the Arch box (12.2)
