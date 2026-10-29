# Alert rules

**File:** `deploy/monitoring/todo/alerts.yaml`  
**Save to:** `~/code/todo/deploy/monitoring/todo/alerts.yaml`

## Goal
Type three alerts: no ready pod, no database, too many 5xx.

## Why
- `for: 2m` waits before firing, so one failed probe during a rollout stays quiet.
- `min(app_database_up)` across pods: one pod unable to reach Postgres is already a problem.
- The error rate divides 5xx requests by all requests over five minutes.

## Help
- Save with `:w! ~/code/todo/deploy/monitoring/todo/alerts.yaml`.
