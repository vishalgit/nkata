# 02 Give the groups something to group

Super agenda sorts your agenda into groups: by deadline, tag, priority,
category and more. Groups need data. A task with no tag, no priority and no
date has nothing to sort by.

## Goal

| Task                       | Priority | Tag     | Deadline    |
|----------------------------|----------|---------|-------------|
| Fix the production outage  | A        | work    | 2026-11-02  |
| Review the pull request    | B        | work    | none        |
| Renew the passport         | none     | home    | 2026-12-15  |
| Clean the downloads folder | C        | home    | none        |

## Steps

For each task:

1. `<S-Up>` on the headline (or `<prefix>,` then the letter) sets the priority.
2. `<prefix>t` sets the tag.
3. `<prefix>d`, `i`, then the date, `<CR>` sets the deadline.

Then open the agenda and watch the tasks fall into groups. Press `<Tab>` on a
header to fold it. Move with `gj` and `gk`.

Spaces: the expected file uses one space before tags and one space in property lines (`:Effort: 4:00`). Set `tags_column = 0` if your tags get pushed to the right.

## Done when

Your file matches `expected.org`.
