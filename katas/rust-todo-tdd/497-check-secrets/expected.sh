#!/usr/bin/env bash
# Test: dev, test and prod each have a database secret, and every value in it is SOPS-encrypted.
set -euo pipefail
cd "$(dirname "$0")/.."
fail=0
for env in dev test prod; do
  f="deploy/k8s/overlays/$env/secret.enc.yaml"
  if [ ! -f "$f" ]; then
    echo "FAIL $env: $f is missing"; fail=1
  elif ! grep -q '^sops:' "$f"; then
    echo "FAIL $env: $f is not a SOPS file"; fail=1
  elif sed -n '/^stringData:/,/^[a-z]/p' "$f" | grep -E '^ +[A-Z_]+:' | grep -vq 'ENC\['; then
    echo "FAIL $env: $f has a plaintext value"; fail=1
  else
    echo "ok   $env"
  fi
done
exit "$fail"
