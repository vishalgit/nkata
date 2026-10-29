#!/usr/bin/env bash
# Check the Arch host for everything this guide expects. Reports only; never installs.
# Usage: scripts/doctor          start-up items must be present; later-chapter items only warn
#        scripts/doctor --all    every item must be present (run before Chapter 12)
set -uo pipefail
ALL=0; [ "${1:-}" = "--all" ] && ALL=1
fail=0 warn=0
ok()   { printf '  \e[32m✓\e[0m %s\n' "$1"; }
bad()  { printf '  \e[31m✗\e[0m %s — %s\n' "$1" "$2"; fail=$((fail + 1)); }
soft() { printf '  \e[33m!\e[0m %s — %s\n' "$1" "$2"; warn=$((warn + 1)); }
# need <now|later> <command> <how to get it>
need() {
  if command -v "$2" >/dev/null 2>&1; then ok "$2"
  elif [ "$1" = now ] || [ "$ALL" = 1 ]; then bad "$2" "$3"
  else soft "$2" "$3"; fi
}
at_least() { [ "$(printf '%s\n%s\n' "$2" "$1" | sort -V | head -1)" = "$2" ]; }   # version $1 >= $2

echo "Start-up tools (Part 0 to Chapter 2a)"
need now git     "sudo pacman -S git"
need now gh      "sudo pacman -S github-cli"
need now jq      "sudo pacman -S jq"
need now docker  "sudo pacman -S docker docker-compose docker-buildx"
need now tmux    "sudo pacman -S tmux"
need now nvim    "sudo pacman -S neovim"
need now curl    "sudo pacman -S curl"
need now openssl "sudo pacman -S openssl"
if command -v nvim >/dev/null 2>&1; then
  v=$(nvim --version | head -1 | grep -oE '[0-9]+\.[0-9]+(\.[0-9]+)?' | head -1)
  if at_least "$v" 0.9; then ok "Neovim $v reads .editorconfig natively"
  else soft "Neovim $v" "0.9 or later reads .editorconfig without a plugin"; fi
fi

echo "Docker"
if docker info >/dev/null 2>&1; then ok "daemon reachable without sudo"
else bad "daemon" "sudo systemctl enable --now docker; sudo usermod -aG docker \$USER, then log in again"; fi
if docker compose version >/dev/null 2>&1; then ok "compose $(docker compose version --short)"
else bad "compose v2" "sudo pacman -S docker-compose"; fi
if docker buildx version >/dev/null 2>&1; then ok "buildx"
else bad "buildx" "sudo pacman -S docker-buildx"; fi

echo "Your user"
if [ "$(id -u)" = 1000 ] && [ "$(id -g)" = 1000 ]; then ok "uid/gid 1000 ($(id -un))"
else bad "uid $(id -u) / gid $(id -g)" "containers run as 1000:1000, so the files they write would not be yours"; fi

echo "GitHub"
if gh auth status >/dev/null 2>&1; then
  ok "gh logged in as $(gh api user --jq .login 2>/dev/null)"
  scopes=$(gh auth status 2>&1 | grep -i 'token scopes' || true)
  for s in repo project; do
    if grep -q "'$s'" <<<"$scopes"; then ok "token scope $s"; else bad "token scope $s" "gh auth refresh -s $s"; fi
  done
  if grep -q "'read:packages'" <<<"$scopes"; then ok "token scope read:packages"
  elif [ "$ALL" = 1 ]; then bad "token scope read:packages" "gh auth refresh -s read:packages"
  else soft "token scope read:packages" "gh auth refresh -s read:packages (Chapter 12)"; fi
else
  bad "gh auth" "gh auth login"
fi

echo "Ports"
for p in 3000 3001 5432 8080; do
  if ss -ltnH "sport = :$p" 2>/dev/null | grep -q .; then
    if docker ps --format '{{.Ports}}' 2>/dev/null | grep -q ":$p->"; then ok "port $p (one of your containers)"
    else soft "port $p" "taken by another process; stop it before the step that publishes it"; fi
  else ok "port $p free"; fi
done

echo "Later chapters"
need later kubectl "sudo pacman -S kubectl (Chapter 2b)"
need later k3d     "sudo pacman -S k3d, or yay -S k3d-bin (Chapter 2b)"
need later helm    "sudo pacman -S helm (Chapter 2d)"
need later k9s     "sudo pacman -S k9s (Chapter 2d)"
need later sops    "sudo pacman -S sops (Chapter 12)"
need later age     "sudo pacman -S age (Chapter 12)"
if command -v sops >/dev/null 2>&1; then
  v=$(sops --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+(\.[0-9]+)?' | head -1)
  if at_least "$v" 3.9; then ok "sops $v supports --filename-override"
  else bad "sops $v" "3.9 or later is needed for bin/new-secret"; fi
fi
if [ -f "$HOME/.config/sops/age/keys.txt" ]; then ok "age key"
elif [ "$ALL" = 1 ]; then bad "age key" "age-keygen -o ~/.config/sops/age/keys.txt (12.4)"
else soft "age key" "created in 12.4"; fi

echo "Optional"
if command -v psql >/dev/null 2>&1 || command -v pg_isready >/dev/null 2>&1; then ok "a Postgres client on the host"
else soft "Postgres client" "used by 1.5 Verify; any client you like, or sudo pacman -S postgresql (do not enable the service)"; fi

echo
if [ "$fail" -gt 0 ]; then echo "$fail missing, $warn for later. Fix the ✗ lines and run again."; exit 1; fi
echo "Ready. $warn item(s) can wait for later chapters."
