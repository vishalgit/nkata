#!/usr/bin/env bash
# Run a command in the dev container as uid 1000.  No arguments: an interactive shell.
set -euo pipefail
cd "$(dirname "$0")/.."
docker compose up -d db dev >/dev/null
[ $# -eq 0 ] && set -- bash
if [ -t 0 ]; then
  exec docker compose exec dev "$@"
else
  exec docker compose exec -T dev "$@"
fi
