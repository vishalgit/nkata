# bin/e2e

**File:** `bin/e2e` (part 1 of 2)  
**Save to:** `~/code/todo/bin/e2e`

## Goal
Type `bin/e2e`: Playwright against the dev server, or against Kubernetes with `E2E_TARGET=k3d`.

## Why
- Against the dev server it uses the Compose service, which reaches `dev:3000` by name.
- Against k3d it uses the host network, so the browser sees `localhost:8080`. From Chapter 7 the production image sets `Secure` cookies, which browsers accept over plain http only on `localhost`.
- Any arguments replace the default command, so `bin/e2e npm install` works too.

## When it passes
```sh
$ chmod +x bin/e2e
$ bin/e2e npm install       # once: node_modules and package-lock.json
```

## Help
- Save with `:w! ~/code/todo/bin/e2e`.
