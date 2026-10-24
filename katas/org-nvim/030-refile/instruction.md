# Refile

## Goal
Move a subtree somewhere else with a few keys.

## What to do
- `<C-c>[` so this file is a refile target.
- On `A stray idea` press `<prefix>r` and pick `Projects`.
- On `Another stray idea` press `<prefix>R`. It copies instead of moving.
- Run `:Org refile_goto_last` to jump to where the last refile went.

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- Refiled entries become the last children of the target.
