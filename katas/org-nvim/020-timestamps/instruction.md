# Timestamps

## Goal
Put dates in your text.

## What to do
- Under `Active date`, open an empty line (`o<Esc>`), press `<prefix>i.` and type `2026-10-30`.
- Under `Inactive date`, same, but `<prefix>i!`.
- Under `Date with time`, press `4<prefix>i.` and type `2026-10-30 14:00`.
- On the meeting date, put the cursor on the month `10` and press `<C-a>`. The weekday fixes itself.
- Then press `<S-Right>` to move one day later.

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- Active `<...>` dates show in the agenda. Inactive `[...]` dates do not.
- Press `.` in the calendar for today. The check uses fixed dates so it works any day.
