# Manifests — Postgres

**File:** `deploy/k8s/postgres.yaml`  
**Save to:** `~/code/todo/deploy/k8s/postgres.yaml`

## Goal
Type the database: a Secret, a Service and a one-replica StatefulSet with its own volume.

## Why
- A StatefulSet, not a Deployment: the pod keeps its name and its volume across restarts.
- `fsGroup: 1000` makes the volume writable by uid 1000, so Postgres runs without root.
- `envFrom: secretRef` hands every key of the Secret to Postgres as environment variables. The app reads only `DATABASE_URL` from the same Secret.
- The plaintext password is for the local cluster only; Chapter 12 replaces it with SOPS-encrypted secrets.

## Help
- Save with `:w! ~/code/todo/deploy/k8s/postgres.yaml`.
