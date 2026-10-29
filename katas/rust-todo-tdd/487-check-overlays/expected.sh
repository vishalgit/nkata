#!/usr/bin/env bash
# Test: dev, test and prod render into their own namespace and host, with no Secret inside.
set -euo pipefail
cd "$(dirname "$0")/.."
fail=0
for env in dev test prod; do
  host=$([ "$env" = prod ] && echo todo.localhost || echo "$env.todo.localhost")
  if ! out=$(kubectl kustomize "deploy/k8s/overlays/$env" 2>&1); then
    echo "FAIL $env: does not render: $out"; fail=1; continue
  fi
  grep -q "namespace: todo-$env" <<<"$out" || { echo "FAIL $env: not in namespace todo-$env"; fail=1; }
  grep -q "host: $host" <<<"$out" || { echo "FAIL $env: no ingress host $host"; fail=1; }
  if grep -q '^kind: Secret' <<<"$out"; then echo "FAIL $env: renders a plaintext Secret"; fail=1; fi
done
if [ "$fail" = 0 ]; then echo "ok: dev, test and prod render"; fi
exit "$fail"
