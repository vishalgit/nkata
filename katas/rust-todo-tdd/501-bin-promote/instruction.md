# Promote one image — bin/promote

**File:** `bin/promote`  
**Save to:** `~/code/todo/bin/promote`

**▶ Task**
```sh
scripts/backlog task TODO-20 12 infra "Promotion pipeline with an approval gate (12.5)" promotion
```

## Goal
`deliver` publishes `ghcr.io/<you>/todo:<sha>` for every commit on `main`. Promotion moves that exact image: pull it, import it into k3d, apply the overlay with the decrypted secret, smoke-test the environment's own URL.

## Why
- Nothing is built here: what reaches prod is byte-for-byte the image that passed the tests.
- The secret is decrypted straight into `kubectl apply`; plaintext never touches the disk.

## When it passes
Try it once by hand with the last commit on `main` (its image is in GHCR). The package is private, so log in with a token that can read packages:

```sh
$ chmod +x bin/promote
$ gh auth refresh -s read:packages
$ gh auth token | docker login ghcr.io -u <you> --password-stdin
$ GITHUB_REPOSITORY_OWNER=<you> bin/promote dev "$(git rev-parse origin/main)"   # ok <- dev runs todo:<sha>
$ curl -s http://dev.todo.localhost:8080/healthz                                  # ok
```

## Help
- Save with `:w! ~/code/todo/bin/promote`.
