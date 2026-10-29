**Epic:** E2 Find my todos · **Type:** story · **Points:** 5

*As a* user with many todos *I want* to see them ten at a time *so that* the page stays fast and readable.

### Acceptance criteria

- [ ] A missing or zero page means page 1; page 3 skips 20 and takes 10 → `a_missing_or_zero_page_is_the_first_page`, `page_three_skips_twenty_and_takes_ten`
- [ ] A page knows its total and its neighbours → `a_page_knows_its_neighbours`, `results_come_ten_at_a_time_with_the_full_total`, `the_list_is_paged`
- [ ] Page links keep the filter and the search → `every_part_is_kept_and_encoded`
- [ ] The pager offers only links that make sense → `the_first_page_offers_next_but_not_previous`, `the_last_page_offers_previous_but_not_next`; journey `long lists come in pages of ten`

### Tasks

- [ ] Ch 4 · `PageRequest`, `Paged<T>`
- [ ] Ch 5 · `LIMIT`/`OFFSET`
- [ ] Ch 7 · `TodoPage`
- [ ] Ch 9 · `list_href`, pager
- [ ] Ch 11 · paging journey
