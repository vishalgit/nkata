# Secrets — test first

**File:** `bin/check-secrets`  
**Save to:** `~/code/todo/bin/check-secrets`

**▶ Task**
```sh
scripts/backlog task TODO-20 12 infra "Encrypted secrets with SOPS and age (12.4)" sops
```

## Goal
Dev, test and prod get random database passwords, stored in the repo **encrypted** with SOPS. Test first: each environment has a secret file, and every value in it is encrypted.

| Secret | Lives in | Who can read it |
| --- | --- | --- |
| `GITHUB_TOKEN` | Created by GitHub per job | That job, until it ends |
| Dev, test, prod database passwords | `deploy/k8s/overlays/<env>/secret.enc.yaml`, SOPS + age | Whoever holds the age key |
| The age private key | `~/.config/sops/age/keys.txt` on the Arch box, plus your password manager | You (and the runners, as you) |
| Playground password `change-me` | `deploy/k8s/overlays/local/secret.yaml`, plaintext | Anyone with the repo; laptop and CI only |
| `RELEASE_TOKEN` | Repository secret (Chapter 2c) | The `release` job only |

## Why
- The age key exists only on your Arch box. GitHub never sees passwords or key: the `home` runner runs as `vishal`, so it finds the key where your shell does.
- The plaintext check looks at every `KEY:` line under `stringData:` and fails if one lacks `ENC[`.

## When it passes
```sh
$ chmod +x bin/check-secrets
$ bin/check-secrets           # FAIL dev: ... is missing (and test, and prod)
```

## Help
- Save with `:w! ~/code/todo/bin/check-secrets`.
