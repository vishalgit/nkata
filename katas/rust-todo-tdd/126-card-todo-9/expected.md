**Epic:** E1 Manage my todos · **Type:** story · **Points:** 3

*As a* user *I want* to fix a todo's title *so that* typos don't stay forever.

### Acceptance criteria

- [ ] Renaming keeps the todo's state → `retitling_keeps_the_state`, `renaming_keeps_a_completed_todo_completed`
- [ ] The event carries the new todo and the old title → `renaming_emits_the_new_todo_and_the_old_title`
- [ ] An unchanged or blank title is refused → `the_same_title_after_trimming_is_unchanged`, `renaming_to_a_blank_title_is_unprocessable`
- [ ] It works over HTTP → `renaming_changes_the_title`

### Tasks

- [ ] Ch 3 · `with_title`
- [ ] Ch 4 · `rename_todo`
- [ ] Ch 5 · `save` (shared with TODO-8)
- [ ] Ch 9 · slice and form
- [ ] Ch 11 · lifecycle journey
