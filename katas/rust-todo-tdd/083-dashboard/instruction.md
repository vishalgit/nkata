# The Todo app dashboard

**File:** `deploy/monitoring/todo/dashboard.yaml`  
**Save to:** `~/code/todo/deploy/monitoring/todo/dashboard.yaml`

## Goal
Type the dashboard ConfigMap: four stats, two time series and a logs panel, loaded automatically by Grafana's sidecar.

## Why
- The label `grafana_dashboard: "1"` is what the sidecar watches for.
- `or vector(0)` shows 0 instead of "No data" when there has never been a 5xx.
- The p95 latency comes from the histogram buckets set in `metrics.rs`.
- It lives in the `monitoring` namespace, next to Grafana.

## Check
The JSON inside must parse — Grafana silently ignores a broken dashboard.
```sh
$ bin/monitoring
```

## When it passes
**◀ PR** `feat(observability): dashboard, scrape config and alerts (TODO-6)` · labels `type:task,layer:observability,epic:E5`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:observability,epic:E5
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

**Then run it, and break it on purpose.** This task has no code, so no branch:

```sh
$ TASK_ONLY=1 scripts/backlog task TODO-6 2 observability "Run it, then break it on purpose (2.27–2.28)"
$ kubectl -n monitoring port-forward svc/kps-grafana 3002:80     # its own tmux window
```
Open `http://localhost:3002` (`admin` / `admin`) and make some traffic:
```sh
$ for i in $(seq 200); do curl -s localhost:8080/healthz >/dev/null; done
$ E2E_TARGET=k3d bin/e2e
```

| You want | Go to |
| --- | --- |
| The health dashboard | Dashboards → **Todo app** |
| Search all app logs | Explore → Loki → `{namespace="todo"} \| json` |
| Only errors | `{namespace="todo", level="ERROR"}` |
| Slow requests | `{namespace="todo"} \| json \| latency_ms > 200` |
| Alert states | Alerting → Alert rules (the three Todo rules) |
| Cluster-wide views | Dashboards → Kubernetes / Compute Resources |

An alert you have never seen fire is a guess:
```sh
$ kubectl -n todo scale statefulset/postgres --replicas=0
```
Within seconds **Database** turns DOWN, the pods go not-ready, and the logs panel shows `health check failed`. After two minutes `TodoDatabaseUnreachable` and `TodoNoReadyPods` fire. Restore it:
```sh
$ kubectl -n todo scale statefulset/postgres --replicas=1
```
The stat returns to UP and the alerts resolve. Close the issue with `gh issue close <n> --comment "Drill passed: DB down → DOWN + alerts firing; restored → resolved."`, then close TODO-6.

**Where to take it next:** notifications (`alertmanager.enabled: true` plus a receiver), traces (`tracing-opentelemetry` and Tempo), a CI test that every metric in `dashboard.yaml` is one the app exports.

**Chapter 2 checkpoint.**
```sh
$ bin/dev cargo t-http      # 5 passed: page, two health, two metrics
$ bin/e2e                   # 1 test x 2 browsers
$ bin/deploy                # ok <- deployed todo:<sha>
```
Milestone *Ch 02 · Walking skeleton* is at 100%: close it and merge the Release PR — your first release, `v0.1.0`, with `todo:v0.1.0` in GHCR. From now on every chapter ends the same way.

## Help
- Save with `:w! ~/code/todo/deploy/monitoring/todo/dashboard.yaml`.
