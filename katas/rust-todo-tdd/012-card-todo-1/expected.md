**Epic:** E0 Delivery platform · **Type:** enabler · **Points:** 3

*As the* developer *I want* the toolchain and the database in uid-1000 containers, beside my own Neovim and tmux on the host, *so that* nothing extra is installed on Arch and every machine builds the same way.

### Acceptance criteria

- [ ] `docker compose exec db id` reports uid 1000, and `.pgdata` belongs to you (1.5)
- [ ] Postgres answers on `127.0.0.1:5432` at the `.env` URL, so host tools reach it (1.5)
- [ ] `bin/dev id` reports uid 1000 (1.6)
