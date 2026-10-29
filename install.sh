#!/usr/bin/env bash
# Installs the nkata runner:
#   kata.lua   -> ~/.config/nkata/kata.lua
#   katas.conf -> ~/.config/nkata/katas.conf   (keyword -> kata folder)
#   nkata      -> ~/.local/bin/nkata
#   katas/*    -> ~/katas/*   (each set is skipped if it already exists)
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"

mkdir -p "$HOME/.config/nkata" "$HOME/.local/bin" "$HOME/katas"
install -m 644 "$here/kata.lua" "$HOME/.config/nkata/kata.lua"
install -m 644 "$here/katas.conf" "$HOME/.config/nkata/katas.conf"
install -m 755 "$here/nkata" "$HOME/.local/bin/nkata"

for set in "$here"/katas/*/; do
  name="$(basename "$set")"
  if [[ -e "$HOME/katas/$name" ]]; then
    echo "~/katas/$name already exists — left untouched"
  else
    cp -r "$set" "$HOME/katas/$name"
    echo "added   ~/katas/$name"
  fi
done

echo
echo "Installed. Available katas (nkata <keyword>):"
n=0
for set in "$here"/katas/*/; do
  name="$(basename "$set")"
  n=$((n + 1))
  kw="$(awk -F= -v f="$name" '
    /^[[:space:]]*(#|$)/ { next }
    { v=$2; gsub(/^[[:space:]]+|[[:space:]]+$/, "", v)
      k=$1; gsub(/^[[:space:]]+|[[:space:]]+$/, "", k)
      if (v == f) { print k; exit } }' "$here/katas.conf")"
  printf '  %d. %-12s %s\n' "$n" "${kw:-(no keyword)}" "$name"
done | LC_ALL=C sort -t. -k1,1n
echo "Add keywords in katas.conf and re-run install.sh."
case ":$PATH:" in
  *":$HOME/.local/bin:"*) ;;
  *) echo "Note: ~/.local/bin is not on your PATH" ;;
esac
