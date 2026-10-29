# README, first push, bootstrap

**File:** `README.md`  
**Save to:** `~/code/todo/README.md`

## Goal
Type the README, then make the only commit that goes straight to `main`, create the private repository and run the bootstrap.

## Then, in the web UI (about ten minutes)
**Project fields.** Rename the built-in Status options to Backlog · Ready · In progress · In review · Done, and add an **Iteration** field (2-week sprints). `gh` cannot edit those.

**Views.**

| View | Layout | Filter | Group / sort |
| --- | --- | --- | --- |
| Board | Board by Status | `label:type:task,type:bug` | Column limit 2 on In progress |
| Backlog | Table | `label:type:story,type:enabler` | Group by Epic; show Points, Sub-issues progress |
| Chapters | Table | `label:type:task` | Group by Milestone; sort by Chapter |
| Sprint | Board | `iteration:@current` | By Status |
| Epics | Table | `label:type:epic` | Sub-issues progress |

**Workflows** (Project → Workflows): auto-add `is:pr is:open` from `todo`; PR linked to issue → In review; item closed and PR merged → Done; auto-archive `is:closed updated:<@today-14d`. Leave "Item added to project" off: it would overwrite the In progress status `scripts/backlog task` sets.

**Security** (Settings → Code security): Dependabot alerts and security updates on, plus secret scanning. **Actions** (Settings → Actions → General): workflow permissions read-only by default; require approval for workflows from outside collaborators.

Part 0 ends with an empty board: settings, labels and fields, no issues.

## When it passes
```sh
$ chmod +x scripts/doctor scripts/bootstrap-github.sh scripts/backlog
$ git add -A && git commit -m "chore: repository, templates and backlog tooling"
$ gh repo create todo --private --source . --push
$ scripts/bootstrap-github.sh        # under a minute; the board is empty
```

## Help
- Save with `:w! ~/code/todo/README.md`.
