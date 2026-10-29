# Test first — bin/check-overlays

**File:** `bin/check-overlays`  
**Save to:** `~/code/todo/bin/check-overlays`

**▶ Task**
```sh
scripts/backlog task TODO-20 12 infra "Kustomize base and overlays, three namespaces (12.3)" overlays
```

## Goal
One cluster, three environments. Test first: a script that renders each overlay and checks what must differ — plus one thing that must never appear, a Secret.

| Environment | Namespace | URL | App pods | Deployed by |
| --- | --- | --- | --- | --- |
| local | `todo` | `http://localhost:8080` | 2 | `bin/deploy`, by hand (and CI's throwaway cluster) |
| dev | `todo-dev` | `http://dev.todo.localhost:8080` | 1 | every merge to `main` |
| test | `todo-test` | `http://test.todo.localhost:8080` | 1 | the pipeline, once dev is healthy; E2E runs here |
| prod | `todo-prod` | `http://todo.localhost:8080` | 2 | the pipeline, after you approve |

## Why
- The environments share the Chapter 2 cluster, each with its own namespace, Postgres and host name; Traefik routes by host. Browsers and curl resolve every `*.localhost` name to your machine, so nothing goes into `/etc/hosts`.
- Every environment runs the same image — the one `deliver` built and tested for that commit; only namespace, host, replicas and secret differ. Kustomize says exactly that: one **base** and one small **overlay** per environment.
- Real secrets arrive encrypted in the next task, so a rendered overlay must contain none.

## When it passes
```sh
$ chmod +x bin/check-overlays
$ bin/check-overlays          # FAIL dev: does not render ... (and test, and prod)
```

## Help
- Save with `:w! ~/code/todo/bin/check-overlays`.
