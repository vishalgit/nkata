# 03 Move a task from the chart

You do not need to leave the chart to change a date. `S` changes the
scheduled day. It uses the same date prompt as `<prefix>s`.

## Goal

Move "Design the logo" to start on **2026-11-09**.

## Steps

1. Open `start.org` and run `:Org timeline`.
2. Warm up: press `+` and `-` to zoom (day, week, month). Press `[` and `]` to
   pan. Then zoom back to the week view.
3. Put the cursor on the row of "Design the logo".
4. Press `S`. In the date prompt press `i`, type `2026-11-09`, press `<CR>`.
5. Press `q` and save with `:w`.

Only the SCHEDULED date of that one task changes.

## Done when

Your file matches `expected.org`.
