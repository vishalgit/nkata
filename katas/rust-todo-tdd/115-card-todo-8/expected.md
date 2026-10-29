**Epic:** E1 Manage my todos · **Type:** story · **Points:** 5

*As a* user *I want* to mark a todo done, and undo that *so that* my list shows what is left.

### Acceptance criteria

- [ ] Completing records when it happened, never before creation → `completing_records_when_it_happened`, `completion_never_precedes_creation`
- [ ] Completing twice, or reopening an active todo, is a business error → `completing_twice_is_an_error`, `reopening_an_active_todo_is_an_error`
- [ ] The state survives a round trip to Postgres → `saving_persists_completion_and_title`
- [ ] Over HTTP: done, then `409` on a second try; reopening works → `completing_a_todo_marks_it_done`, `completing_twice_is_a_conflict`, `a_completed_todo_can_be_reopened`
- [ ] Each item offers Done or Undo → `an_active_item_offers_done_and_a_completed_one_offers_undo`

### Tasks

- [ ] Ch 3 · states
- [ ] Ch 4 · workflows
- [ ] Ch 5 · `save`
- [ ] Ch 9 · slices
- [ ] Ch 9 · buttons
- [ ] Ch 11 · lifecycle journey
