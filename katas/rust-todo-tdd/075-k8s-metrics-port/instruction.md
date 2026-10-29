# Name the ports in the manifests

**File:** `deploy/k8s/app.yaml` (part 2 of 2)  
**Save to:** `~/code/todo/deploy/k8s/app.yaml`

## Goal
Name the container's two ports, and append a metrics-only Service.

## Why
- Prometheus finds targets by Service labels and named ports.
- The metrics Service keeps publishing pods that are **not ready** (`publishNotReadyAddresses`). A normal Service drops them, so Prometheus would lose sight of the app exactly when the database is down.
- The Ingress still routes only to the `todo` Service on port 80, so `/metrics` stays inside the cluster.

## Check
```sh
$ bin/dev cargo t-http
$ bin/deploy
$ kubectl -n todo logs deploy/todo | head -3      # JSON lines now
```

## When it passes
**◀ PR** `feat(observability): JSON logs and a separate metrics port (TODO-5)` · labels `type:task,type:enabler,layer:observability,epic:E5` · closes the task and TODO-5

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,type:enabler,layer:observability,epic:E5
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/deploy/k8s/app.yaml`.
