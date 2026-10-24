# Links

## Goal
Link anything to anything.

## What to do
- Under `Web`, open an empty line (`o<Esc>`), press `<prefix>li`, type `https://orgmode.org`, description `the Org manual`.
- On `Tables` press `<prefix>ls` (store a link).
- Under `Heading`, open an empty line, `<prefix>li`, pick the stored link, description `the tables`.
- Press `<CR>` on each link to follow it. `<C-o>` comes back.

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- `<prefix>lt` shows the raw link text. `<prefix>ln` / `<prefix>lp` jump between links.
- If the stored link comes with a `file:...::` part, delete it by hand so it reads `[[*Tables]...`.
