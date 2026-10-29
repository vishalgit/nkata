# Protect main — the ruleset as code

**File:** `.github/rulesets/main.json`  
**Save to:** `~/code/todo/.github/rulesets/main.json`

## Goal
Now that `test`, `deliver` and `pr-title` exist, type the ruleset: `main` accepts only squash-merged pull requests whose checks passed, from a branch up to date with `main`.

## Run first
```sh
$ git switch main && git pull --ff-only && git switch -c todo-4/main-ruleset
```

## Why
- Zero required approvals, because GitHub never lets you approve your own PR: the checks and your own deliberate merge are the gate.
- `strict_required_status_checks_policy`: the branch must be up to date with `main`, so the checks ran against what will actually land.
- The guardrails in this part are small `ci:` and `chore:` PRs under TODO-4 (`Part of #<TODO-4>`, label `type:chore`) — no task issue of their own.

## When it passes
Apply it once (your GitHub Pro plan covers rulesets on a private repo):

```sh
$ gh api -X POST "repos/$(gh repo view --json nameWithOwner --jq .nameWithOwner)/rulesets" \
    --input .github/rulesets/main.json
```
From here on a red pipeline blocks the merge button.

**◀ PR** `chore: main ruleset as code (TODO-4)` · labels `type:chore,epic:E0` · body: `Part of #<TODO-4>`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:chore,epic:E0
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/.github/rulesets/main.json`.
