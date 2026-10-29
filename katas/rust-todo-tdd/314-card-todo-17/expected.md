**Epic:** E1 Manage my todos · **Type:** story · **Points:** 3

*As a* user *I want* my todos listed, newest first, as soon as I open the app *so that* I see my day at a glance.

### Acceptance criteria

- [ ] The list endpoint returns a page of my todos → `a_created_todo_is_returned_and_listed`
- [ ] Items carry BEM classes, and an empty list shows a hint → `an_item_shows_its_title_with_bem_classes`, `a_completed_item_gets_the_modifier_class`, `an_empty_list_shows_a_friendly_hint`, `a_list_renders_one_item_per_todo`
- [ ] The board is rendered on the server with my data → `a_logged_in_user_gets_their_todos_rendered_on_the_server`; journey `todos survive a full page reload`

### Tasks

- [ ] Ch 7 · list endpoint
- [ ] Ch 8 · components
- [ ] Ch 8 · board
- [ ] Ch 11 · reload journey
