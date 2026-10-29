# E2E against any environment

**File:** `bin/e2e` (part 2 of 2)  
**Save to:** `~/code/todo/bin/e2e`

## Goal
Let `bin/e2e` take the URL from `E2E_BASE_URL`.

## Why
The host network still makes `Secure` cookies work on `*.localhost`.

## Check
```sh
$ E2E_TARGET=k3d E2E_BASE_URL=http://dev.todo.localhost:8080 bin/e2e     # the journeys, against dev
```

## Help
- Save with `:w! ~/code/todo/bin/e2e`.
