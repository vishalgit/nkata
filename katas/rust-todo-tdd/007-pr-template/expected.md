Part of #<story issue>  <!-- "Closes #<story>" too on the PR that finishes the story's last task -->
Closes #<task issue>

**Task:** Ch <n> · <task> (<section>)

**Red → green:** <the test(s) written first>

### Definition of Done
- [ ] Every acceptance-criterion test for this task exists and passes
- [ ] `cargo fmt` and `cargo lint` are clean; `.sqlx/` is current
- [ ] Pipeline green on this PR
- [ ] After merge: dev and test are green; approve prod once test looks right
- [ ] Dashboard shows no new errors
