# Green — the playground's secret moves to a local overlay

**File:** `deploy/k8s/overlays/local/secret.yaml`  
**Save to:** `~/code/todo/deploy/k8s/overlays/local/secret.yaml`

## Goal
Move today's manifests into `base/`, then give the plaintext Secret its own file in a `local` overlay that keeps the playground and CI's throwaway cluster working.

## Run first
```sh
$ mkdir -p deploy/k8s/base deploy/k8s/overlays/{local,dev,test,prod}
$ git mv deploy/k8s/{namespace,postgres,app,kustomization}.yaml deploy/k8s/base/
```

## Why
The base must hold nothing environment-specific — a password least of all.

## Help
- Instead of typing it, you can produce it from the old file: `sed -n '1,/^---$/p' deploy/k8s/base/postgres.yaml | sed '$d' > deploy/k8s/overlays/local/secret.yaml`.
- Save with `:w! ~/code/todo/deploy/k8s/overlays/local/secret.yaml`.
