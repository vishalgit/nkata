**Epic:** E0 Delivery platform · **Type:** enabler · **Points:** 8

*As the* developer *I want* every merge to roll through dev and test before prod, with encrypted secrets and my approval as the last gate, *so that* nothing reaches prod that I have not seen working.

### Acceptance criteria

- [ ] Each environment renders into its own namespace and host, with no plaintext Secret → `bin/check-overlays` (12.3)
- [ ] Dev, test and prod each have a database secret, and every one is encrypted → `bin/check-secrets` (12.4)
- [ ] A merge to `main` deploys to dev, then to test, runs the E2E suite against test, and waits for an `approve` comment before prod (12.5)
- [ ] A `deny` comment stops the run, and prod keeps the version it had (12.5)

### Tasks

- [ ] Ch 12 · Kustomize base and overlays
- [ ] Ch 12 · SOPS and age
- [ ] Ch 12 · promotion pipeline
