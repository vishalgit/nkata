# bin/monitoring

**File:** `bin/monitoring`  
**Save to:** `~/code/todo/bin/monitoring`

## Goal
Type `bin/monitoring`: install or upgrade the three Helm releases, then the app's own monitoring resources.

## Why
- `helm upgrade --install` is re-runnable: it installs the first time and upgrades after.
- `create namespace --dry-run=client -o yaml | kubectl apply -f -` is the idempotent way to make a namespace.
- The app's folder is applied only if it has files, so this works before the next task adds them.

## When it passes
```sh
$ chmod +x bin/monitoring
$ bin/monitoring                       # first run: a few minutes
$ kubectl -n monitoring get pods       # all Running
```

**◀ PR** `feat(observability): Prometheus, Grafana, Loki and Alloy (TODO-6)` · labels `type:task,layer:observability,epic:E5`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:observability,epic:E5
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Chart values change between majors. If an install fails on an unknown key, compare with `helm show values grafana/loki` (or the other chart) and adjust.
- Save with `:w! ~/code/todo/bin/monitoring`.
