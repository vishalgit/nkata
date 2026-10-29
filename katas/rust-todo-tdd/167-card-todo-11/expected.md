**Epic:** E2 Find my todos · **Type:** story · **Points:** 5

*As a* user *I want* to search my todos by title *so that* I can find one quickly in a long list.

### Acceptance criteria

- [ ] Blank means no search, terms are trimmed, and over 100 characters is refused → `blank_input_means_no_search`, `a_term_is_trimmed`, `an_overlong_term_is_rejected`, `an_overlong_search_is_unprocessable`
- [ ] Matching is case-insensitive, by part of the title, and only over my todos → `search_matches_part_of_a_title_for_one_owner_newest_first`
- [ ] `%` and `_` match literally → `like_wildcards_in_user_input_are_escaped`, `wildcards_typed_by_the_user_match_literally`
- [ ] Results are rendered on the server, including "no matches" → `a_search_is_rendered_on_the_server`
- [ ] The search box keeps the term and the filter → `the_search_box_keeps_the_term_and_the_current_filter`; journey `search narrows the list and keeps working with filters`

### Tasks

- [ ] Ch 4 · `SearchTerm`, `TodoQuery`
- [ ] Ch 5 · trigram index and SQL search
- [ ] Ch 7 · search over HTTP
- [ ] Ch 9 · search box
- [ ] Ch 11 · search journey
