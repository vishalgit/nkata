# Change a TODO state

## Goal
Move tasks through their states.

## What to do
- On `Cycle my state` press `cit` a few times, then `ciT` to go back. End with `<prefix>T` and `d` (DONE).
- On `Write the report` press `cit` once. It becomes NEXT.
- On `Old idea` press `<prefix>T` and `c` (CANCELLED).

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- Marking DONE adds a `CLOSED: [today]` stamp. The check can't know today's time, so remove it: `:%s/CLOSED: \[[^]]*\]\s*//` then `:g/^\s*$/d`.
- The states come from the `#+TODO:` line. Everything after `|` counts as done.
