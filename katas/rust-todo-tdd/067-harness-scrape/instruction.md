# The harness scrapes metrics

**File:** `crates/server/tests/common/mod.rs` (part 3 of 7)  
**Save to:** `~/code/todo/crates/server/tests/common/mod.rs`

**▶ Task**
```sh
scripts/backlog task TODO-5 2 observability "Metrics endpoint, middleware and health gauge (2.21–2.22)" metrics
```

## Goal
Metrics get their own port (9000), so the ingress never exposes them. Tests call the metrics router directly: add `scrape_metrics` at the end of the harness.

## Why
A free function, not a `TestApp` method: the metrics are global to the process, not to one app.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/common/mod.rs`.
