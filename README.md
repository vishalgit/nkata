# nkata

A 4-pane kata runner for Neovim with a Claude Code coach. You type into the
editor until it matches the final result, then move on to the next kata.

```
┌────────────────┬────────────────┐
│ 1 Editor       │ 3 Instruction  │
│                │   + status     │
├────────────────┼────────────────┤
│ 2 Final result │ 4 Claude Code  │
│   (read-only)  │   (terminal)   │
└────────────────┴────────────────┘
```

All four panes are kept the same size. When your terminal or Neovim window is
resized, the grid re-equalizes.

## Requirements

- Neovim 0.10 or newer
- A Neovim profile at `~/.config/kickstart/`. nkata starts Neovim with
  `NVIM_APPNAME=kickstart`, the same as a `kvim` alias would.
- `~/.local/bin` on your `PATH`
- [Claude Code](https://code.claude.com/docs/en/overview) (`claude` on your
  `PATH`) for pane 4. Without it, the other three panes work as normal and
  pane 4 says Claude wasn't found.

## Installation

From the folder you extracted:

```bash
./install.sh
```

This installs:

| Path | What it is |
| --- | --- |
| `~/.local/bin/nkata` | Launcher |
| `~/.config/nkata/kata.lua` | Runner |
| `~/katas/org-basics/` | Sample org.nvim kata set |
| `~/katas/leptos-scss-design-system/` | Leptos + SCSS design system kata set |

Each kata set is copied only if `~/katas` doesn't already have one with that
name, so running `install.sh` again updates the launcher and runner but never
overwrites your sets or their progress.

If `~/.local/bin` isn't on your `PATH`, the installer tells you. Add this to
your shell config:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

## Usage

```bash
nkata gtd                      # keyword from katas.conf; resume at the first unsolved kata
nkata ~/katas/org-basics 2      # start at kata #2
nkata ~/katas/org-basics 003    # start at the kata whose folder starts with 003
nkata --help
```

### Keymaps

| Keys | Action |
| --- | --- |
| `<leader>kn` | Next kata |
| `<leader>kp` | Previous kata |
| `<leader>kg` | Go to a kata by number or name prefix (you're prompted) |
| `<leader>kr` | Reset the editor to blank or the starter text |
| `<leader>kw` | Write a copy of the editor to a path you type (see below) |
| `<leader>kx` | Run the visually selected text (or, in normal mode, the current line) in your shell |
| `<leader>kc` | Jump into the Claude pane (restarts Claude if it has exited) |
| `<leader>ke` | Jump back to the editor |
| `<leader>kq` | Save progress and quit |
| `<M-h>` `<M-j>` `<M-k>` `<M-l>` | From any pane: move left / down / up / right, like vim's `<C-w>hjkl` (landing in Claude resumes typing) |
| `<M-i>` | From any pane: jump to the instruction pane |
| `<M-s>` | From any pane: jump to the solution (final result) pane |
| `<M-y>` | From any pane: close the focused pane (not the editor); the others grow into its space |
| `<M-u>` | From any pane: restore every closed pane |

The same actions exist as commands: `:KataNext`, `:KataPrev`, `:KataGoto [n]`,
`:KataReset`, `:KataCopy [path]`, `:KataRun [cmd]`, `:KataClaude`, `:KataEditor`,
`:KataKill`, `:KataRestore`, `:KataQuit`.

### Closing and restoring panes

`<M-y>` closes the focused pane so the remaining panes grow into the space.
A closed pane stays closed when you move between katas; only `<M-u>` brings
it back, refilled for the current kata.

- The editor pane can't be closed.
- Closing the Claude pane only hides it. The session keeps running, and
  restoring shows the same conversation.
- Your editor text is untouched by closing and restoring.
- `<M-i>` and `<M-s>` do nothing for a closed pane except say how to
  restore it.

### Running a command from the editor

Select text in any pane (`v`, `V` or `Ctrl-v`) and press `<leader>kx`. The
selection runs in your shell (`'shell'`, normally `$SHELL`), from the folder you
launched nkata in. In normal mode, `<leader>kx` runs the line under the cursor.

- A multi-line selection runs as one script, so `cd`, variables and
  pipes carry from line to line.
- Output streams into a floating window as the command runs. stderr shows in
  red, and the last line shows the exit code (green for 0).
- `q` or `Esc` closes the window. If the command is still running, closing
  the window stops it.
- `:KataRun ls -la` runs a command you type; `:'<,'>KataRun` runs a line range.

The command runs non-interactively (no stdin), so commands that wait for input
see end-of-file. Use the Claude pane or a real terminal for interactive commands.

### Copying your attempt out

`<leader>kw` asks `Copy editor to:` with a path pre-filled as
`<current folder>/<kata-name>.<ext>`. Edit it (Tab completes paths), press
Enter, and whatever is in the editor pane right now is written there, saved
or not.

- `~` is expanded and missing parent folders are created.
- A path ending in `/`, or an existing folder, gets the default
  `<kata-name>.<ext>` file name.
- If the file already exists you're asked before it's overwritten.
- `Esc` or an empty path cancels.

The editor keeps working on its own scratch file, so the copy doesn't change
the kata. `:KataCopy ~/notes/headings.org` does the same without the prompt.

The `<leader>k…` keymaps are normal-mode only. Inside the Claude pane you're in
terminal mode, where `<leader>` is typed into Claude, so use `<M-h>`/`<M-j>`/`<M-k>`/`<M-l>` to move
out. Change the Alt keys with `nav_keys` / `to_instr_key` / `to_solution_key` at the top
of `kata.lua`; avoid keys Claude Code uses itself, such as `Esc`, `Ctrl-c`,
`Ctrl-d` and `Ctrl-r`.

### The Claude pane

Claude Code starts once per session in pane 4, with the kata set folder as its
working directory, and keeps the same conversation as you move between katas.

- **It knows the current kata.** nkata keeps a small file describing the
  current kata and its file paths, and gives Claude standing rules to read it
  before answering. When you switch katas, nkata types a one-line
  `[nkata] Now on kata …` notice into Claude so it catches up.
- **Hints, not answers.** Claude can compare your attempt with the final
  result, but gives nudges unless you explicitly ask for the answer.
- **Saving a good hint.** Ask "save that hint" (or similar). Claude adds it as
  a bullet under `## Help` in the kata's `instruction.md`. Pane 3 updates
  straight away and the hint is there every time you run that kata.
- **Editing katas.** Claude may change the current kata's `instruction.md`,
  `expected.*` and `start.*`. Panes 2 and 3 refresh as soon as the files change
  on disk; a new starter applies on the next `<leader>kr`. Claude is told not
  to touch other katas, `.progress.json` or your attempt unless you ask.

Moving to another kata sends nothing to Claude except `/clear`, which Claude
Code handles locally, so navigation costs no tokens and old katas drop out of
the context. Type `kata?` (optionally followed by a question) in pane 4 when you
want Claude to read the current kata and give a hint. To make Claude announce
each switch instead, set `notify_claude_on_switch = true` (this costs a model
turn per switch); `clear_claude_on_switch = false` stops the clearing.

### How a kata passes

The editor is compared with the final result as you type. Trailing spaces and
trailing blank lines don't count. When they match, pane 3 shows a green
PASSED banner with your time and the separators turn green.

Progress is saved to `.progress.json` inside each kata set. Relaunching
resumes at the first unsolved kata.

## Writing your own katas

A kata set is any folder of kata folders. They run in folder-name order.

```
~/katas/my-set/
  001-first/
    instruction.md     required — shown in pane 3
    expected.rs        required — the final result; the extension sets the filetype
    start.rs           optional — starter text for the editor (blank if absent)
  002-second/
    ...
```

## Configuration

| Setting | Where | Default |
| --- | --- | --- |
| Neovim profile | `NKATA_NVIM_APPNAME` env var | `kickstart` |
| Runner location | `NKATA_HOME` env var | `~/.config/nkata` |
| Claude command | `NKATA_CLAUDE_CMD` env var, or `claude_cmd` in `kata.lua` | `claude` |
| Kata-switch notices to Claude | `notify_claude_on_switch` in `kata.lua` | off |
| Clear Claude on kata switch | `clear_claude_on_switch` in `kata.lua` | on |
| Keymap prefix | `prefix` at the top of `kata.lua` | `<leader>k` |
| Pane-hop keys | `nav_keys` / `to_instr_key` / `to_solution_key` in `kata.lua` | `<M-h/j/k/l>` / `<M-i>` / `<M-s>` |
| Close / restore pane keys | `kill_pane_key` / `restore_key` in `kata.lua` | `<M-y>` / `<M-u>` |
| Scratch file folder | `tmp_root` at the top of `kata.lua` | `/tmp/nkata` |
| Pass/fail colours | `pass_color` / `fail_color` in `kata.lua` | green / red |

While nkata is running, `vim.g.kata_mode` is set before your `init.lua` loads.
Your config can check it to skip dashboards or session restore:

```lua
if not vim.g.kata_mode then
  -- dashboard / session plugins here
end
```

## Uninstallation

```bash
./uninstall.sh
```

This removes:

| Path | What it is |
| --- | --- |
| `~/.local/bin/nkata` | Launcher |
| `~/.config/nkata/` | Runner |
| `/tmp/nkata/` | Editor scratch files and Claude session files from past sessions |

Your kata lessons in `~/katas`, including `org-basics` and every
`.progress.json`, are left untouched. To remove those as well, delete them by
hand:

```bash
rm -rf ~/katas/org-basics
```
