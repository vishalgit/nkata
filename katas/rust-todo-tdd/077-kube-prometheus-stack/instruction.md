# Helm values — Prometheus and Grafana

**File:** `deploy/monitoring/kube-prometheus-stack.yaml`  
**Save to:** `~/code/todo/deploy/monitoring/kube-prometheus-stack.yaml`

**▶ Task**
```sh
scripts/backlog task TODO-6 2 observability "Install Prometheus, Grafana, Loki and Alloy (2.25)" monitoring-stack
```

## Goal
Install Helm, then type the values for kube-prometheus-stack: Prometheus, Grafana, kube-state-metrics and node-exporter.

## Run first
```sh
$ sudo pacman -S helm
$ mkdir -p deploy/monitoring/todo
```

## Why
- k3s runs etcd, the scheduler, the controller manager and kube-proxy inside one process; scraping them separately only produces errors.
- The three `…NilUsesHelmValues: false` lines make Prometheus pick up ServiceMonitors and rules from **every** namespace, including `todo`.
- The Grafana sidecar loads any ConfigMap labelled `grafana_dashboard: "1"` — that is how the app's dashboard arrives.
- Loki is added as a second data source with a fixed `uid`, so the dashboard can refer to it.

## Help
- Save with `:w! ~/code/todo/deploy/monitoring/kube-prometheus-stack.yaml`.
