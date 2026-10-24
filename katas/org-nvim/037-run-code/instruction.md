# Run code

## Goal
Run code inside your notes and keep the output.

## What to do
- Put the cursor in the Lua block, press `<prefix>'`. Change `7` to `9`, `:w`, close it.
- Press `<C-c><C-c>` in each block. It asks first, then writes a RESULTS block.
- Jump to the Python block with `<prefix>bn`, and remove its result with `<prefix>bk`.

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- `<prefix>bp` jumps to the previous block.
- The check expects one blank line, then `#+RESULTS:`, then `: value`.
