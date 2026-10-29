# The e2e service

**File:** `compose.yaml` (part 3 of 3)  
**Save to:** `~/code/todo/compose.yaml`

## Goal
Add the `e2e` service to `compose.yaml`, under `services:`.

## Why
- Playwright ships browser builds only for Ubuntu, so this is the one image on a distribution you did not choose. It still runs as uid 1000.
- `profiles: ["e2e"]`: `docker compose up` leaves it alone; `bin/e2e` runs it on demand.
- `ipc: host` gives Chromium enough shared memory.
- `BASE_URL` defaults to the dev container by its service name.

## Help
- Save with `:w! ~/code/todo/compose.yaml`.
