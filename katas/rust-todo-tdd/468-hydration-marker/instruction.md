# A hydration marker

**File:** `crates/app/src/app.rs` (part 7 of 7)  
**Save to:** `~/code/todo/crates/app/src/app.rs`

**▶ Task**
```sh
scripts/backlog task TODO-14 11 e2e "Registration journey (11.3)" registration-journey
```

## Goal
↪ Completes **TODO-14 · Create an account** — the registration journey. The Playwright scaffold has run since the walking skeleton; this chapter adds the journeys that matter, one per PR. First, tell tests when the app is interactive.

| Layer | Count so far (approx.) | Speed | Catches |
| --- | --- | --- | --- |
| Domain unit + property | ~50 | milliseconds | wrong business rules |
| Persistence integration | ~12 | ~50 ms each | wrong SQL, mapping, constraints |
| Component (SSR render) | ~12 | milliseconds | wrong markup, missing classes |
| HTTP integration | ~32 | ~100 ms each | wiring, auth, status codes |
| E2E (this chapter) | ~10 | seconds each | hydration, JS behaviour, real browsers |

With E2E tests, "red" usually means a journey the UI cannot complete. The UI exists here, so run each journey as you write it; if it fails, decide whether the app or the test is wrong — and fix the app unless the test is.
## Why
- If a test clicks before the WASM bundle has hydrated, the browser does a plain form post instead of the Leptos action, and the test flakes.
- Effects never run on the server, and this one is compiled only into the browser build: the attribute appears exactly when the page is interactive.
- Each journey is the **last task** of its stories, so these PRs close stories — and, as their last stories close, epics.

## Help
- Save with `:w! ~/code/todo/crates/app/src/app.rs`.
