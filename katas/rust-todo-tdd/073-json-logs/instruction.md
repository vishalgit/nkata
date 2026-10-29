# Serve metrics on port 9000 and log as JSON

**File:** `crates/server/src/main.rs` (part 2 of 3)  
**Save to:** `~/code/todo/crates/server/src/main.rs`

**▶ Task**
```sh
scripts/backlog task TODO-5 2 observability "JSON logs, metrics port and manifests (2.23–2.24)" json-logs
```

## Goal
Log as JSON when `LOG_FORMAT=json`, and start a second listener for `/metrics` before the main one.

## Run first
```sh
$ bin/dev cargo add -p server tracing-subscriber --features json
```

## Why
- Log aggregators index fields, not prose: in JSON, `level`, `path` and `latency_ms` become searchable.
- Locally the dev server keeps readable logs; only the production image sets `LOG_FORMAT=json`.
- The metrics listener runs in its own task on its own port; the Ingress never routes to it.

## Check
```sh
$ bin/dev cargo t-http
$ bin/dev curl -s localhost:9000/metrics | grep http_requests_total   # with cargo leptos watch running
```

## Help
- Save with `:w! ~/code/todo/crates/server/src/main.rs`.
