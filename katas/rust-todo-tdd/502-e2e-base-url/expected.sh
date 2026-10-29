#!/usr/bin/env bash
# Usage: bin/e2e                    -> all E2E tests against the dev server (cargo leptos watch)
#        E2E_TARGET=k3d bin/e2e     -> against the k3d deployment on http://localhost:8080
#        bin/e2e npx playwright test tests/smoke.spec.ts --project=chromium
set -euo pipefail
cd "$(dirname "$0")/.."
if [ $# -eq 0 ]; then set -- npx playwright test; fi
if [ "${E2E_TARGET:-dev}" = "k3d" ]; then
  # Host network, so the browser sees "localhost": Secure cookies are only accepted there over plain http.
  source .env
  exec docker run --rm --network host --ipc host --user 1000:1000 \
    -e BASE_URL="${E2E_BASE_URL:-http://localhost:8080}" -e HOME=/tmp \
    -v "$PWD/end2end:/e2e" -w /e2e \
    "mcr.microsoft.com/playwright:v${PLAYWRIGHT_VERSION}-noble" "$@"
fi
exec docker compose --profile e2e run --rm e2e "$@"
