**Epic:** E3 My account · **Type:** story · **Points:** 8

*As a* visitor *I want* to register with my email and a password *so that* my todos are mine.

### Acceptance criteria

- [ ] Emails are normalised and sanity-checked → `normalises_case_and_whitespace`, `rejects_addresses_missing_a_part`, `rejects_malformed_domains`
- [ ] Passwords are 12–128 characters and never printed → `rejects_short_passwords`, `debug_output_never_shows_the_secret`
- [ ] Every problem is reported at once → `all_problems_are_reported_together`, `an_invalid_registration_lists_every_problem`
- [ ] Passwords are stored as Argon2 hashes, and an email registers only once → `a_hash_verifies_the_original_password_only`, `the_same_email_cannot_register_twice`, `an_email_can_only_register_once`
- [ ] Registering logs you in → `registering_returns_the_user_and_starts_a_session`; journey `registering lands you on your empty board`

### Tasks

- [ ] Ch 6 · types
- [ ] Ch 6 · `register_user`
- [ ] Ch 6 · users table, Argon2, `insert_user`
- [ ] Ch 7 · endpoint
- [ ] Ch 8 · page
- [ ] Ch 11 · journey
