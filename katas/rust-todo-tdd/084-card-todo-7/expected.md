**Epic:** E1 Manage my todos · **Type:** story · **Points:** 5

*As a* logged-in user *I want* to add a todo with a short title *so that* I can capture what I need to do.

### Acceptance criteria

- [ ] A title is trimmed and must be 1–120 characters → `trims_surrounding_whitespace`, `rejects_an_empty_title`, `rejects_one_character_over_the_maximum`, `counts_characters_not_bytes`
- [ ] A valid command creates an active todo with the given id, owner and time → `a_valid_command_produces_a_created_event`
- [ ] It is stored and can be found again → `an_inserted_todo_can_be_found_again`
- [ ] Only a logged-in user can add one, and a blank title is refused → `creating_a_todo_requires_a_login`, `an_empty_title_is_unprocessable`
- [ ] It works in the browser → journey `the full lifecycle of a todo`

### Tasks

- [ ] Ch 3 · typed ids and title
- [ ] Ch 4 · `create_todo`
- [ ] Ch 5 · table, `insert`, `find`
- [ ] Ch 7 · server function
- [ ] Ch 8 · add form
- [ ] Ch 11 · lifecycle journey
