# Helm values — Alloy

**File:** `deploy/monitoring/alloy.yaml`  
**Save to:** `~/code/todo/deploy/monitoring/alloy.yaml`

## Goal
Type Alloy's values: it tails every pod's logs through the Kubernetes API, parses the JSON level, and pushes to Loki.

## Why
- One Deployment reading logs through the API needs no host mounts, unlike a DaemonSet tailing files on each node.
- The pipeline reads top to bottom: discover pods → copy namespace, pod, container and app into labels → read their logs → pull `level` out of the JSON into a label → push to Loki.
- `level` as a label is what makes `{namespace="todo", level="ERROR"}` fast.

## Help
- Save with `:w! ~/code/todo/deploy/monitoring/alloy.yaml`.
