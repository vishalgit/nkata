# Helm values — Loki

**File:** `deploy/monitoring/loki.yaml`  
**Save to:** `~/code/todo/deploy/monitoring/loki.yaml`

## Goal
Type Loki's values: one small process with local storage.

## Why
- `SingleBinary` runs every Loki component in one pod. The read/write/backend replicas and the caches are for big clusters, so they are switched off.
- Logs are kept 72 hours on a 2 GiB volume.

## Help
- Save with `:w! ~/code/todo/deploy/monitoring/loki.yaml`.
