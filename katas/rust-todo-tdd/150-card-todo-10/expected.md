**Epic:** E2 Find my todos · **Type:** story · **Points:** 3

*As a* user *I want* to show All, Active or Completed todos *so that* I can focus on what is left.

### Acceptance criteria

- [ ] Unknown or missing filters fall back to All → `parses_known_values_and_falls_back_to_all`
- [ ] Each filter selects the right todos → `matches_by_state`, `a_filter_says_which_completion_state_it_wants`
- [ ] The filter runs in SQL and combines with search → `the_filter_combines_with_the_search`, `search_and_filter_work_together_over_http`
- [ ] The tabs work in the browser → journey `filters show the right subset`

### Tasks

- [ ] Ch 4 · `Filter`
- [ ] Ch 5 · SQL condition
- [ ] Ch 7 · list parameter
- [ ] Ch 9 · tabs
- [ ] Ch 11 · filters journey
