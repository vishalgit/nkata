# The scrape config — ServiceMonitor

**File:** `deploy/monitoring/todo/servicemonitor.yaml` (part 1 of 2)  
**Save to:** `~/code/todo/deploy/monitoring/todo/servicemonitor.yaml`

**▶ Task**
```sh
scripts/backlog task TODO-6 2 observability "ServiceMonitor, alert rules and dashboard (2.26)" dashboard
```

## Goal
Type the ServiceMonitor: scrape the `todo-metrics` Service's `metrics` port every 15 seconds.

## Why
These resources stay outside `deploy/k8s/` on purpose: they need the monitoring stack's custom resource types, which CI's throwaway cluster lacks.

## Help
- Save with `:w! ~/code/todo/deploy/monitoring/todo/servicemonitor.yaml`.
