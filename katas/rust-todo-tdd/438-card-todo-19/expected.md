**Epic:** E4 Privacy and roles · **Type:** story · **Points:** 5

*As an* admin *I want* to see every todo and delete abusive ones, without editing anyone's words, *so that* I can keep the app clean.

### Acceptance criteria

- [ ] Owners edit; admins may delete anything; only admins view all → `only_owners_may_edit_whatever_their_role`, `deleting_is_for_owners_and_admins`, `only_admins_may_view_all_todos`
- [ ] The same rules hold over HTTP → `an_admin_can_delete_anyones_todo`, `an_admin_cannot_edit_someone_elses_todo`, `only_admins_see_all_todos`
- [ ] The admin page shows owners and is closed to everyone else → `each_row_shows_the_owner_and_a_delete_button`, `the_admin_page_is_closed_to_ordinary_users`

### Tasks

- [ ] Ch 10 · policy
- [ ] Ch 10 · policy-based access
- [ ] Ch 10 · admin endpoint, table and page
