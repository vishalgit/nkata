**Epic:** E0 Delivery platform · **Type:** enabler · **Points:** 5

*As the* developer *I want* a tiny, root-free production image deployed to a local cluster *so that* what I test is what I run.

### Acceptance criteria

- [ ] The image is built `FROM scratch` and runs as uid 1000 (2.13)
- [ ] `bin/deploy` rolls out two pods and `/healthz` on `localhost:8080` answers `ok` (2.16)
- [ ] `kubectl -n todo exec deploy/todo -- sh` fails: there is no shell in the image (2.16)
