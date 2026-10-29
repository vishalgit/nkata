**Epic:** E3 My account · **Type:** story · **Points:** 8

*As a* registered user *I want* to log in and out *so that* only I can use my account.

### Acceptance criteria

- [ ] A wrong password and an unknown email get the same answer → `an_unknown_user_gets_the_very_same_error`, `a_wrong_password_and_an_unknown_email_look_the_same`
- [ ] Sessions use long random tokens, stored only as hashes, in a locked-down cookie → `session_tokens_are_long_and_random`, `a_token_is_stored_as_a_stable_hash_never_as_itself`, `the_session_cookie_is_locked_down`
- [ ] Expired or deleted sessions identify nobody → `an_expired_or_deleted_session_resolves_to_nobody`
- [ ] Login, `me` and logout work over HTTP → `logging_in_with_the_right_password_starts_a_session`, `me_returns_the_user_behind_the_cookie`, `logging_out_ends_the_session`
- [ ] Visitors are sent to the login page → `anonymous_visitors_are_sent_to_the_login_page`, `the_login_page_renders_a_working_form`; journeys `logging out and back in`, `a wrong password shows a generic error`

### Tasks

- [ ] Ch 6 · `authenticate`
- [ ] Ch 6 · sessions, tokens, cookies, session store
- [ ] Ch 7 · endpoints
- [ ] Ch 8 · redirect, login page, header
- [ ] Ch 11 · journeys
