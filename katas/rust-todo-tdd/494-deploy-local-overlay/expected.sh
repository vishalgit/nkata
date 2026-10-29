#!/usr/bin/env bash
# Build the production image, load it into k3d, roll it out, and smoke-test it.
# Usage: bin/deploy [tag]     (default tag: current git commit)
set -euo pipefail
cd "$(dirname "$0")/.."
TAG="${1:-$(git rev-parse --short HEAD)}"
CLUSTER="${K3D_CLUSTER:-todo}"

docker build -t "todo:$TAG" .
k3d image import "todo:$TAG" -c "$CLUSTER"
kubectl kustomize deploy/k8s/overlays/local | sed "s|image: todo:dev|image: todo:$TAG|" | kubectl apply -f -
kubectl -n todo rollout status statefulset/postgres --timeout=180s
kubectl -n todo rollout status deployment/todo --timeout=180s

for i in $(seq 1 30); do
  if curl -fsS http://localhost:8080/healthz; then echo " <- deployed todo:$TAG"; exit 0; fi
  sleep 2
done
echo "smoke test failed" >&2
exit 1
