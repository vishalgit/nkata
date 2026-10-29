# CI/CD — pipeline.yml

**File:** `.github/workflows/pipeline.yml` (part 1 of 7)  
**Save to:** `~/code/todo/.github/workflows/pipeline.yml`

## Goal
Type the pipeline: a `test` job in the same containers as your laptop, then a `deliver` job that deploys to a throwaway k3d cluster.

## Why
| Stage | Proves |
| --- | --- |
| EditorConfig, fmt, lint, deny | Formatting, clippy rules, safe and licensed dependencies |
| HTTP tests (unit from Ch 3, integration from Ch 5) | Behaviour, against a real Postgres |
| E2E | A real browser works with the hydrated app |
| deliver: build | The `scratch` image builds |
| deliver: deploy + smoke | The image runs in Kubernetes as uid 1000 and reaches its database |
| deliver: publish | `main` produces an image in GHCR, tagged by commit |

- The runner's user is not uid 1000, so the job hands the checkout to uid 1000 first and back at the end (`if: always()`).
- `--wait` makes `compose up` return only once the db healthcheck passes.
- `cargo leptos serve` runs detached; the next step waits up to 20 minutes for the first compile to answer `/healthz`.
- `${GITHUB_REPOSITORY_OWNER,,}` lower-cases the owner: GHCR image names must be lower case.

## Check
The PR's own run is the acceptance test: watch `test` and `deliver` go green. The first run builds the dev image from scratch and takes a while.

## When it passes
**◀ PR** `ci: test and deliver pipeline (TODO-4)` · labels `type:task,layer:infra,epic:E0`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:infra,epic:E0
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- If the EditorConfig step flags a line you cannot fix (a long URL in Rust), exclude that one file in `.editorconfig-checker.json`, not the rule for everyone.
- Save with `:w! ~/code/todo/.github/workflows/pipeline.yml`.
