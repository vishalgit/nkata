# Custom commands and stuck projects

## Goal
Build your own agenda page.

## What to do
- Type the `agenda` config shown in the final result pane.
- Put it inside your `require("org").setup({ ... })` table. Restart `kvim`.
- `<prefix>a` then `w`: three views in one buffer (Today, Open work tasks, Waiting for).
- `<prefix>a` then `u`: urgent things.
- `<prefix>a` then `#`: "stuck" projects, tagged `project` with no NEXT or TODO child.

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- The plugin's `examples/minimal_init.lua` has these set up already.
- To see a stuck project, make an org file with `* Learn Rust :project:` and nothing under it.
