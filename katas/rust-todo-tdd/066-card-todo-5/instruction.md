# Story card TODO-5 · The app reports on itself

**File:** `backlog/stories/TODO-5.md`  
**Save to:** `~/code/todo/backlog/stories/TODO-5.md`

## Goal
Write the card for **TODO-5 · The app reports on itself** (E5 Operate the app, 3 points).

Deploying is half the loop; knowing whether the deployment is healthy is the other half. Added now, while the app is tiny, every later feature ships with metrics, structured logs, a dashboard and alerts already in place.

```text
 todo pods ──/metrics :9000──▶ Prometheus ◀── kube-state-metrics (pod ready, restarts)
     │                              │
     └─ stdout JSON logs ─▶ Alloy ─▶ Loki
                                    │
                          Grafana ◀─┴─ dashboard "Todo app" + alert rules
```

| Option | Setup | Shows | When |
| --- | --- | --- | --- |
| `kubectl` | none | pod status, events, one pod's logs | one-off checks |
| **k9s** | `pacman -S k9s` | live health, restarts, logs, describe | everyday first look |
| **Grafana + Prometheus + Loki** | 3 Helm releases, ~1–1.5 GB RAM | dashboards over time, log search, alerts | the real "is it healthy?" answer |

## Why
- The card is the issue body: `scripts/backlog story` reads `backlog/stories/TODO-5.md`.
- Each acceptance criterion names the test that will prove it. The story closes when every one exists and passes.
- Commit the card with the story's first task.

## When it passes
Create the issues from this card:

```sh
$ scripts/backlog epic E5 "Operate the app" "Know at a glance whether production is healthy"
$ scripts/backlog story TODO-5 E5 enabler 3 "The app reports on itself"
```

A new epic starts here, so its issue comes first.

Then the first task, which has no code — so no branch:

```sh
$ TASK_ONLY=1 scripts/backlog task TODO-5 2 observability "k9s for a first look (2.20)"
$ sudo pacman -S k9s
$ k9s -n todo
```

| Key | Does |
| --- | --- |
| `:pods` / `:deploy` | List pods or deployments; READY and RESTARTS are your health status |
| `l` on a pod | Stream its logs (`0` = all containers) |
| `d` | Describe: probe failures and events |
| `s` | Shell — fails for the app, which has no shell |
| `:ns` then `monitoring` | Watch the monitoring stack (later in this chapter) |

k9s keeps no history and reads one pod's logs at a time; the rest of this part fixes both. Close the issue once you have used it:
`gh issue close <n> --comment "Used k9s against the todo namespace."`

## Help
- Save with `:w! ~/code/todo/backlog/stories/TODO-5.md`.
