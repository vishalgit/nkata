# Story card TODO-20 · Changes reach prod only through dev, test and my approval

**File:** `backlog/stories/TODO-20.md`  
**Save to:** `~/code/todo/backlog/stories/TODO-20.md`

## Goal
Write the card for **TODO-20 · Changes reach prod only through dev, test and my approval** (E0 Delivery platform, 8 points).

The app is feature-complete and has shipped through the same pipeline since Chapter 2. This chapter closes the loop from GitHub to the k3d cluster on your Arch box: every merge rolls through dev and test, secrets are encrypted in the repo, nothing reaches prod until you approve it, and the last PR declares 1.0.0.

| Stage | Added in | Runs |
| --- | --- | --- |
| EditorConfig, fmt, lint, cargo-deny | Ch 2c | every PR and push |
| Unit, integration/component, HTTP tests | Ch 2–5 | every PR and push |
| Migrate, offline query data check | Ch 5 | every PR and push |
| E2E (Playwright, 2 browsers) | Ch 2c, journeys in Ch 11 | every PR and push |
| Build `scratch` image, throwaway k3d, smoke `/healthz` | Ch 2c | after tests pass |
| Publish image to GHCR; release-please | Ch 2c | `main` only |
| Deploy configuration checks | Ch 12 | every PR and push |
| Promote to dev, test (+ E2E), your approval, prod | Ch 12 | `main` only |

**First, finish TODO-4 — two self-hosted runners.** No code, so no branch:

```sh
$ TASK_ONLY=1 scripts/backlog task TODO-4 12 infra "Self-hosted runners on the home machine (12.2)"
```

GitHub's runners cannot reach your laptop. A **self-hosted runner** can: it polls GitHub, so no ports open, and it runs as `vishal` (uid 1000) with your Docker, k3d and kubectl. You need two: `home` deploys; `gate` only waits for your approval, so an approval left open overnight never blocks the next deploy.

In the repository: **Settings → Actions → Runners → New self-hosted runner**, Linux x64. Run the download commands it shows in `~/actions-runner`, then:

```sh
$ cd ~/actions-runner
$ ./config.sh --url https://github.com/<you>/todo --token <token> --name home --labels home --unattended
```

Click **New self-hosted runner** again for a fresh token, and set up `gate` from the same download:

```sh
$ mkdir ~/actions-runner-gate && cd ~/actions-runner-gate
$ tar xzf ~/actions-runner/actions-runner-linux-x64-*.tar.gz
$ ./config.sh --url https://github.com/<you>/todo --token <token> --name gate --labels gate --unattended
```

Install both as systemd services that run as you, never as root:

```sh
$ cd ~/actions-runner && sudo ./svc.sh install vishal && sudo ./svc.sh start
$ cd ~/actions-runner-gate && sudo ./svc.sh install vishal && sudo ./svc.sh start
```

Keep the repository **private** while they are attached: on a public repository a stranger's pull request could run code on your machine. When both show Idle: `gh issue close <n> --comment "Runners home and gate are Idle."`, then close TODO-4.

## Why
- The card is the issue body: `scripts/backlog story` reads `backlog/stories/TODO-20.md`.
- Each acceptance criterion names the test that will prove it. The story closes when every one exists and passes.
- Commit the card with the story's first task.

## When it passes
Create the issues from this card:

```sh
$ scripts/backlog story TODO-20 E0 enabler 8 "Changes reach prod only through dev, test and my approval"
```

## Help
- Save with `:w! ~/code/todo/backlog/stories/TODO-20.md`.
