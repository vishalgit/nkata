# One entry point into the container — bin/dev

**File:** `bin/dev`  
**Save to:** `~/code/todo/bin/dev`

**▶ Task**
```sh
scripts/backlog task TODO-1 1 infra "bin/dev and the tmux session (1.6–1.7)" bin-dev
```

## Goal
Type `bin/dev`: every cargo, sqlx and bacon command in this guide runs through it.

## Run first
```sh
$ mkdir -p bin
```

## Why
- `docker compose up -d` first, so `bin/dev` works even after a reboot.
- `-t 0` asks "is stdin a terminal?". Interactive use gets a TTY; piped or scripted use gets `-T`, which `exec` needs when there is no terminal.
- `exec` replaces the shell, so the exit code of your command is the exit code of `bin/dev`.

## When it passes
```sh
$ chmod +x bin/dev
$ bin/dev id                        # uid=1000(vishal) gid=1000(vishal)
```

## Help
- Save with `:w! ~/code/todo/bin/dev`.
