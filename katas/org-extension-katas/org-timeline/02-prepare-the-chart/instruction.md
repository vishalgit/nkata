# 02 Give tasks dates and effort

The timeline draws a bar from the SCHEDULED day to the DEADLINE. A task needs
at least one of: SCHEDULED, DEADLINE or Effort. Clocked days can show too.

## Goal

| Task            | Scheduled    | Deadline     | Effort |
|-----------------|--------------|--------------|--------|
| Write the report | 2026-11-02  | 2026-11-06   | 4:00   |
| Design the logo  | 2026-11-04  | 2026-11-13   | 2:00   |
| Launch the site  | 2026-11-09  | 2026-11-20   | 1:00   |

## Steps

For each task:

1. `<prefix>s`, then `i`, type the scheduled date, `<CR>`.
2. `<prefix>d`, then `i`, type the deadline, `<CR>`.
3. `<prefix>xe` and pick the effort. (The choices come from `Effort_ALL` in
   the file header.)

Then run `:Org timeline` and look at the three bars.

Org puts both dates on one line, the deadline first, then the schedule.

Spaces: the expected file uses one space before tags and one space in property lines (`:Effort: 4:00`). Set `tags_column = 0` if your tags get pushed to the right.

## Done when

Your file matches `expected.org`.
