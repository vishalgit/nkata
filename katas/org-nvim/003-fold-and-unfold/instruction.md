# Fold and unfold

## Goal
Show and hide parts of the file without thinking.

## What to do
- The file opens folded. On a headline press `<Tab>`: folded, children, everything.
- Press `<S-Tab>` to cycle the whole file: overview, contents, show all.
- Press `3<S-Tab>` to show headlines down to level 3. Then `5<S-Tab>`.
- Rename `Level 5` to `Level 5 found`.
- On `Another level 2`, fold just the drawer: put the cursor on `:PROPERTIES:` and press `<Tab>`.
- `Old notes` is archived, so it stays shut. Open it with `<C-c><C-Tab>` and change `hidden` to `found`.

## Help
- Only the text is checked. The folding is how you get there.
- Go back to fully folded with `<S-Tab>` before you start each round.
