# Install and check

## Goal
Type the org.nvim setup for your kickstart config, then install it and check it is healthy.

## What to do
- Type the plugin setup shown in the final result pane. It uses `vim.pack` (Neovim 0.12+).
- Copy it into your kickstart config, for example `~/.config/kickstart/plugin/org.lua`.
- Restart `kvim` and run `:checkhealth org`. Fix anything red.

## Shell
```sh
mkdir -p ~/org
touch ~/org/refile.org ~/org/todo.org
```

## Help
- On lazy.nvim instead? The spec is `{ "xheisenbugx/org.nvim", main = "org", lazy = false, opts = { ... } }`.
- Want to try it without touching your config? From the plugin repo run:
  `nvim -u examples/minimal_init.lua examples/tutorial.org`
- After install, open `~/org/todo.org`, type `* TODO Buy milk` and press `<leader>os` to schedule it.
