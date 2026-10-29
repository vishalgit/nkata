# Automatic releases — release-please config

**File:** `.github/release-please-config.json`  
**Save to:** `~/code/todo/.github/release-please-config.json`

## Goal
Type the release-please configuration: which commit types make which changelog sections.

## Run first
```sh
$ git switch main && git pull --ff-only && git switch -c todo-4/release-please
```

## Why
release-please reads the Conventional Commit titles landing on `main`, works out the next semantic version, and keeps one **Release PR** open with that version and the new `CHANGELOG.md` entries. Merging the Release PR creates the tag, the GitHub Release and the versioned image.

| PR title (squash commit) | Changelog section | Bump before 1.0.0 | From 1.0.0 |
| --- | --- | --- | --- |
| `feat: …` | Features | minor | minor |
| `fix:`, `perf:`, `revert:` | Bug fixes, Performance, Reverts | patch | patch |
| `deps: …` (Dependabot) | Dependencies | patch | patch |
| `feat!: …` or `BREAKING CHANGE:` footer | ⚠ BREAKING | minor | major |
| `refactor`, `test`, `ci`, `build`, `docs`, `chore` | hidden | none | none |

- `bump-minor-pre-major` keeps a breaking change from jumping to 1.0.0 while the app is young; 1.0.0 is a deliberate step in Chapter 12.
- The `simple` type keeps the version in `version.txt` next to `CHANGELOG.md`, both written only by Release PRs. The crates stay at `0.1.0` with `publish = false`, so `Cargo.lock` never changes in a Release PR.

## Help
- Save with `:w! ~/code/todo/.github/release-please-config.json`.
