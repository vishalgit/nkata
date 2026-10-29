# One formatting baseline — .editorconfig

**File:** `.editorconfig`  
**Save to:** `~/code/todo/.editorconfig`

## Goal
Install the host tools, create the repository folders, then type `.editorconfig`: the first file, so every file after it is born formatted.

## Run first
```sh
$ sudo pacman -S git github-cli jq
$ gh auth login
$ gh auth refresh -s project        # the project commands need this scope
$ cd ~/code/todo
$ git init -b main
$ mkdir -p .github/ISSUE_TEMPLATE .github/workflows backlog/stories docs/domain-stories docs/adr scripts
$ touch docs/domain-stories/.gitkeep docs/adr/.gitkeep
```

## Why
- Neovim 0.9+ reads `.editorconfig` natively; no plugin needed.
- Its rules already name files that arrive later (Rust, SCSS, migrations, SOPS secrets). A rule for a missing file costs nothing.
- `unset` for generated and encrypted files means "leave them exactly as their tools wrote them".
- The CI check that enforces it joins the pipeline in Chapter 2c.

## Help
- Save with `:w! ~/code/todo/.editorconfig`.
