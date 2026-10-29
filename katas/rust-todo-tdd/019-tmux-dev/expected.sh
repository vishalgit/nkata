#!/usr/bin/env bash
# Open (or re-attach to) the "todo" tmux session on the host.
set -euo pipefail
cd "$(dirname "$0")/.."
S=todo
docker compose up -d db dev
if ! tmux has-session -t "$S" 2>/dev/null; then
  tmux new-session -d -s "$S" -n editor -c "$PWD"
  tmux send-keys   -t "$S:editor" 'nvim .' C-m
  tmux new-window  -t "$S" -n server -c "$PWD"   # bin/dev cargo leptos watch
  tmux new-window  -t "$S" -n tests  -c "$PWD"   # bin/dev bacon <job>
  tmux new-window  -t "$S" -n db     -c "$PWD"
  tmux send-keys   -t "$S:db" 'docker compose exec db psql -U todo -d todo_dev' C-m
  tmux new-window  -t "$S" -n shell  -c "$PWD"   # git, gh, bin/dev, kubectl
  tmux select-window -t "$S:editor"
fi
if [ -n "${TMUX:-}" ]; then tmux switch-client -t "$S"; else tmux attach -t "$S"; fi
