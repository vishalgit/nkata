**Epic:** E4 Privacy and roles · **Type:** story · **Points:** 5

*As a* user *I want* nobody else to see or change my todos *so that* I can trust the app with my data.

### Acceptance criteria

- [ ] Anonymous requests are refused → `creating_a_todo_requires_a_login`
- [ ] Lists only ever show the caller's todos → `users_only_see_their_own_todos`
- [ ] Someone else's todo does not exist for you (`404`, not `403`) → `another_users_todo_does_not_exist_for_you`, `nobody_can_rename_someone_elses_todo`
- [ ] Two browsers never see each other's data → journey `one user never sees another user's todos`

### Tasks

- [ ] Ch 7 · `require_user` and the owner-scoped list
- [ ] Ch 9 · `access.rs`
- [ ] Ch 10 · policy
- [ ] Ch 11 · isolation journey
