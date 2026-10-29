# CI checks the secrets too

**File:** `.github/workflows/pipeline.yml` (part 6 of 7)  
**Save to:** `~/code/todo/.github/workflows/pipeline.yml`

## Goal
Extend the Deploy configuration step.

## When it passes
**◀ PR** `feat(k8s): SOPS-encrypted secrets for dev, test and prod (TODO-20)` · labels `type:task,layer:infra,epic:E0`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:infra,epic:E0
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/.github/workflows/pipeline.yml`.
