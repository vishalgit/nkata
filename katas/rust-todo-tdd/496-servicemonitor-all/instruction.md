# Prometheus sees every environment

**File:** `deploy/monitoring/todo/servicemonitor.yaml` (part 2 of 2)  
**Save to:** `~/code/todo/deploy/monitoring/todo/servicemonitor.yaml`

## Goal
Under `spec:` add a `namespaceSelector` for any namespace, then `kubectl apply -f deploy/monitoring/todo/`.

## Why
The dashboard and alerts still read `namespace="todo"`; exercise 8 at the end of this chapter makes them per environment.

## When it passes
**◀ PR** `feat(k8s): base and overlays for dev, test and prod (TODO-20)` · labels `type:task,layer:infra,epic:E0`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:infra,epic:E0
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/deploy/monitoring/todo/servicemonitor.yaml`.
