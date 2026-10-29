# Manifests — the namespace

**File:** `deploy/k8s/namespace.yaml`  
**Save to:** `~/code/todo/deploy/k8s/namespace.yaml`

**▶ Task**
```sh
scripts/backlog task TODO-3 2 infra "Manifests and bin/deploy (2.15–2.16)" deploy
```

## Goal
Type the namespace every app resource lives in.

## Run first
```sh
$ mkdir -p deploy/k8s
```

## Why
A namespace groups the app's resources, so `kubectl -n todo get all` shows exactly them, and Chapter 12 can run three copies side by side.

## Help
- Save with `:w! ~/code/todo/deploy/k8s/namespace.yaml`.
