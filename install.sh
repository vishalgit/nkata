#!/usr/bin/env bash
# Installs the nkata runner:
#   kata.lua   -> ~/.config/nkata/kata.lua
#   nkata      -> ~/.local/bin/nkata
#   katas/*    -> ~/katas/*   (each set is skipped if it already exists)
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"

mkdir -p "$HOME/.config/nkata" "$HOME/.local/bin" "$HOME/katas"
install -m 644 "$here/kata.lua" "$HOME/.config/nkata/kata.lua"
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

echo "Installed. Try:  nkata ~/katas/org-basics"
case ":$PATH:" in
  *":$HOME/.local/bin:"*) ;;
  *) echo "Note: ~/.local/bin is not on your PATH" ;;
esac
