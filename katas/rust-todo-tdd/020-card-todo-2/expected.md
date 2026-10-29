**Epic:** E0 Delivery platform · **Type:** enabler · **Points:** 3

*As the* developer *I want* a server-rendered, hydrated page and an honest health check *so that* every later feature has a working path from browser to database.

### Acceptance criteria

- [ ] The home page renders its heading → `the_home_page_renders` (retired in Chapter 8, when `/` becomes private)
- [ ] Health is `200 ok` when Postgres answers → `health_is_ok_when_the_database_answers`
- [ ] Health is `503` when Postgres is down → `health_is_503_when_the_database_is_down`
