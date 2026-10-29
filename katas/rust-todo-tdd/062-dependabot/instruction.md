# Dependabot

**File:** `.github/dependabot.yml`  
**Save to:** `~/code/todo/.github/dependabot.yml`

## Goal
Cargo, npm, Docker images and Actions now all exist: type the Dependabot configuration.

## Run first
```sh
$ git switch main && git pull --ff-only && git switch -c todo-4/dependabot
```

## Why
- Grouping keeps the Leptos crates moving together, as they must.
- The commit prefix decides the release: `deps:` is a patch release, `ci:` none.
- Dependabot PRs pass the same checks; merge them with `gh pr merge --squash` once green. When it bumps `@playwright/test`, bump `PLAYWRIGHT_VERSION` in `.env` in the same PR.

## When it passes
**◀ PR** `ci: Dependabot for cargo, npm, docker and actions (TODO-4)` · labels `type:chore,epic:E0` · body: `Part of #<TODO-4>`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:chore,epic:E0
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/.github/dependabot.yml`.
