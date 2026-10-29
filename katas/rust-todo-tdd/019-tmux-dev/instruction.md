# A tmux session on the host

**File:** `scripts/tmux-dev.sh`  
**Save to:** `~/code/todo/scripts/tmux-dev.sh`

## Goal
Type `scripts/tmux-dev.sh`: it opens (or re-attaches to) the `todo` session with one window per job.

## Why
- It uses your own tmux config and prefix. Windows are addressed by name (`todo:editor`), so your `base-index` does not matter.
- Inside tmux it switches client; outside it attaches.

| Window | Purpose |
| --- | --- |
| editor | Neovim on the host |
| server | `bin/dev cargo leptos watch` (from Chapter 2) |
| tests | `bin/dev bacon <job>` (from Chapter 2) |
| db | `psql` inside the db container |
| shell | git, gh, `bin/dev`, `bin/deploy`, `kubectl` |

## When it passes
```sh
$ chmod +x scripts/tmux-dev.sh
$ scripts/tmux-dev.sh
```

**◀ PR** `build: bin/dev and the tmux session (TODO-1)` · labels `type:task,type:enabler,layer:infra,epic:E0`. The body says `Closes #<task>` and `Closes #<TODO-1>`: this PR finishes the story.

**Checkpoint.** Milestone *Ch 01 · Workbench* is at 100%: close it.

## Help
- Save with `:w! ~/code/todo/scripts/tmux-dev.sh`.
