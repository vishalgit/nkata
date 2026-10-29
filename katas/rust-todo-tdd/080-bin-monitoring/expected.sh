#!/usr/bin/env bash
# Install or upgrade Prometheus + Grafana, Loki and Alloy in the k3d cluster, then the app's
# dashboard, scrape config and alert rules.
set -euo pipefail
cd "$(dirname "$0")/.."
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts >/dev/null
helm repo add grafana https://grafana.github.io/helm-charts >/dev/null
helm repo update >/dev/null
kubectl create namespace monitoring --dry-run=client -o yaml | kubectl apply -f -

helm upgrade --install kps prometheus-community/kube-prometheus-stack -n monitoring \
  -f deploy/monitoring/kube-prometheus-stack.yaml --wait --timeout 10m
helm upgrade --install loki grafana/loki -n monitoring \
  -f deploy/monitoring/loki.yaml --wait --timeout 10m
helm upgrade --install alloy grafana/alloy -n monitoring \
  -f deploy/monitoring/alloy.yaml --wait --timeout 5m

if ls deploy/monitoring/todo/*.yaml >/dev/null 2>&1; then kubectl apply -f deploy/monitoring/todo/; fi
echo "Grafana: kubectl -n monitoring port-forward svc/kps-grafana 3002:80  ->  http://localhost:3002 (admin/admin)"
