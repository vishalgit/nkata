#!/usr/bin/env bash
# Roll a published image out to one environment and smoke-test it.
# Usage: bin/promote dev|test|prod <git sha>     (needs a GHCR login, sops and the age key)
set -euo pipefail
cd "$(dirname "$0")/.."
ENV="${1:?usage: bin/promote dev|test|prod <sha>}"
SHA="${2:?usage: bin/promote dev|test|prod <sha>}"
case "$ENV" in
  dev|test) HOST="$ENV.todo.localhost" ;;
  prod)     HOST="todo.localhost" ;;
  *)        echo "unknown environment: $ENV" >&2; exit 2 ;;
esac
NS="todo-$ENV"
OWNER="${GITHUB_REPOSITORY_OWNER:?set GITHUB_REPOSITORY_OWNER to your GitHub user}"
IMAGE="ghcr.io/${OWNER,,}/todo:$SHA"
CLUSTER="${K3D_CLUSTER:-todo}"

docker pull "$IMAGE"
docker tag "$IMAGE" "todo:$SHA"
k3d image import "todo:$SHA" -c "$CLUSTER"

kubectl create namespace "$NS" --dry-run=client -o yaml | kubectl apply -f -
sops --decrypt "deploy/k8s/overlays/$ENV/secret.enc.yaml" | kubectl -n "$NS" apply -f -
kubectl kustomize "deploy/k8s/overlays/$ENV" | sed "s|image: todo:dev|image: todo:$SHA|" | kubectl apply -f -
kubectl -n "$NS" rollout status statefulset/postgres --timeout=180s
kubectl -n "$NS" rollout status deployment/todo --timeout=180s

for i in $(seq 1 30); do
  if curl -fsS "http://$HOST:8080/healthz"; then echo " <- $ENV runs todo:$SHA"; exit 0; fi
  sleep 2
done
echo "smoke test failed on $ENV" >&2
exit 1
