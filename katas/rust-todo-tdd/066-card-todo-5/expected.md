**Epic:** E5 Operate the app · **Type:** enabler · **Points:** 3

*As the* operator *I want* request metrics, a database-health metric and structured logs *so that* I can tell how the app behaves without reading its code.

### Acceptance criteria

- [ ] Requests are counted by route and status → `requests_are_counted_on_the_metrics_endpoint`
- [ ] Database health is exported as a gauge → `database_health_is_exported_as_a_gauge`
- [ ] Pods log one JSON line per request, and `/metrics` is not reachable through the ingress (2.25–2.26)
