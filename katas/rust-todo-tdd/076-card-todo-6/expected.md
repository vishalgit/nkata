**Epic:** E5 Operate the app · **Type:** enabler · **Points:** 5

*As the* operator *I want* a dashboard, searchable logs and alerts *so that* I learn about problems before users do.

### Acceptance criteria

- [ ] The "Todo app" dashboard shows ready pods, database status, request rate, latency and logs (2.29)
- [ ] `{namespace="todo"} | json` finds every pod's logs in one place (2.29)
- [ ] Scaling Postgres to zero turns the Database stat DOWN and fires `TodoDatabaseUnreachable` within about two minutes (2.30)
