# Story card TODO-1 · A reproducible workbench

**File:** `backlog/stories/TODO-1.md`  
**Save to:** `~/code/todo/backlog/stories/TODO-1.md`

## Goal
Write the card for **TODO-1 · A reproducible workbench** (E0 Delivery platform, 3 points).

Your Neovim and tmux stay on Arch; Docker runs the Rust toolchain and Postgres as uid 1000, so every file a container writes is yours. At the end of this chapter one script opens a tmux session and your host tools can reach the project database.

| Thing | Runs on | Why |
| --- | --- | --- |
| Neovim, rust-analyzer, tmux | Host | Your existing setup |
| cargo, cargo-leptos, sqlx-cli, bacon | `dev` container (`rust:latest`, uid 1000) | Reproducible toolchain |
| PostgreSQL | `db` container (`postgres:latest`, uid 1000) | Published on `127.0.0.1:5432` for Neovim |
| Playwright browsers | `e2e` container (uid 1000) | Chapter 2 |
| Kubernetes | k3d on the host's Docker | Chapter 2 |

## Why
- The card is the issue body: `scripts/backlog story` reads `backlog/stories/TODO-1.md`.
- Each acceptance criterion names the test that will prove it. The story closes when every one exists and passes.
- Commit the card with the story's first task.

## When it passes
Create the issues from this card:

```sh
$ scripts/backlog epic E0 "Delivery platform" "Every push tested, built, deployed and verified"
$ scripts/backlog story TODO-1 E0 enabler 3 "A reproducible workbench"
```

A new epic starts here, so its issue comes first.

## Help
- Save with `:w! ~/code/todo/backlog/stories/TODO-1.md`.
