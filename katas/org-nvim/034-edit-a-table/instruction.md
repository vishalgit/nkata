# Edit a table

## Goal
Type rough tables, let Org make them neat.

## What to do
- Put the cursor in the messy table and press `<C-c><C-c>`. It aligns.
- Put the cursor in the Name column and sort with `<prefix>Ts`, then `a`.
- Put the cursor in the Language column and press `<M-l>` to move it to the end.
- Select the three CSV lines with `V` and press `<prefix>Tc`.

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- `<M-k>` / `<M-j>` move rows. In Insert mode, `|-` then `<Tab>` draws a line.
- `<S-CR>` on a number copies it down, adding one each time.
