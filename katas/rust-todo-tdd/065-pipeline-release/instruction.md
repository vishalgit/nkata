# The release job

**File:** `.github/workflows/pipeline.yml` (part 2 of 7)  
**Save to:** `~/code/todo/.github/workflows/pipeline.yml`

## Goal
Make the release token, then append the `release` job: on `main`, after `deliver` published the image, it only re-tags it.

**The release token.** PRs and tags made with the built-in `GITHUB_TOKEN` start no workflows, so a Release PR made that way would never run `test` and `deliver`, and the ruleset would keep it unmergeable. Make a fine-grained token:

1. GitHub → Settings → Developer settings → Fine-grained tokens → **Generate new token**: name `todo release-please`, expiry one year at most, **Only select repositories** → `todo`.
2. Repository permissions: **Contents**, **Pull requests** and **Issues** read and write. Nothing else.
3. `gh secret set RELEASE_TOKEN` and paste it.
4. Put a calendar reminder a week before it expires. An expired token fails the `release` job; deploys carry on regardless (Chapter 12).

## Why
- A release never builds anything new: `imagetools create` gives the image `deliver` already tested a second tag, the version.
- `concurrency: release` keeps two merges from racing each other's Release PR.
- `outputs.tag` is read by the prod approval in Chapter 12.

## When it passes
**◀ PR** `ci: automatic versions and changelog with release-please (TODO-4)` · labels `type:chore,epic:E0` · body: `Part of #<TODO-4>`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:chore,epic:E0
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

After this merges, a Release PR appears (`chore(main): release 0.1.0`) and updates itself on every merge. Its title passes `pr-title`.

**Releasing, in practice.** When a chapter's milestone reaches 100%, close it, read the Release PR, merge it:

```sh
$ gh pr list --label "autorelease: pending"        # the Release PR
$ gh pr checks <n> --watch && gh pr merge <n> --squash --delete-branch
$ gh release view                                   # notes, tag, date
```

To correct a changelog line after a merge, edit the merged PR's body and add the replacement message between `BEGIN_COMMIT_OVERRIDE` and `END_COMMIT_OVERRIDE`; the open Release PR picks it up.

**Weekly, ten minutes:** merge or close Dependabot PRs, groom the Backlog view (split anything over 8 points), and check the Sprint view for what slipped.

## Help
- Save with `:w! ~/code/todo/.github/workflows/pipeline.yml`.
