# Add headlines

## Goal
Create headlines fast.

## What to do
- On `Fruit` press `<M-CR>` and type `Vegetables`. A sibling appears.
- Press `<Esc>`, then `<M-CR>` again. Still in Insert mode, press `<Tab>`: the new headline becomes a child. Type `Carrots`.
- On `Grains` press `<M-S-CR>` and type `Buy rice`. That is a TODO sibling.
- On `Food` press `<prefix>ih` and type `Drinks`. A new top-level heading.

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- `<Tab>` in Insert mode on an empty headline keeps cycling its level.
- Also try `<prefix>it` (TODO heading) and `<prefix>is` (subheading), then `u`.
