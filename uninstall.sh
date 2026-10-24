#!/usr/bin/env bash
# Uninstalls the nkata runner: the launcher and the installed runner files.
# Your kata lessons in ~/katas (org-basics and any others, with their
# .progress.json files) are never touched.
#
# Removes:
#   ~/.local/bin/nkata     launcher
#   ~/.config/nkata/       runner (kata.lua)
#   /tmp/nkata/            editor scratch files from past sessions
set -euo pipefail

removed=0
for path in "$HOME/.local/bin/nkata" "$HOME/.config/nkata" "/tmp/nkata"; do
  if [[ -e "$path" || -L "$path" ]]; then
    rm -rf -- "$path"
    echo "removed  $path"
    removed=$((removed + 1))
  fi
done

if [[ $removed -eq 0 ]]; then
  echo "Nothing to remove — nkata isn't installed."
else
  echo "nkata uninstalled. Your kata lessons in ~/katas are untouched."
fi
