# Green — the metrics module

**File:** `crates/server/src/metrics.rs`  
**Save to:** `~/code/todo/crates/server/src/metrics.rs`

## Goal
Add the metrics facade and the Prometheus exporter, then type `metrics.rs`: the recorder, the router that serves it, and the middleware that records every request.

## Run first
```sh
$ bin/dev cargo add -p server metrics
$ bin/dev cargo add -p server metrics-exporter-prometheus --no-default-features
```

## Why
- The `metrics` facade records values; the Prometheus exporter renders them. `--no-default-features` takes only its recorder, not its HTTP server: you serve `/metrics` yourself.
- `OnceLock`: a process can install only one global recorder. Each test binary is a process, so the first `handle()` call installs it and every later one reuses it.
- `MatchedPath` is the route pattern (`/api/todos/create`), not the raw URL: label cardinality stays low however many ids appear in URLs.
- The middleware also writes one structured log line per request — the JSON logs of the next task.

## Help
- Save with `:w! ~/code/todo/crates/server/src/metrics.rs`.
