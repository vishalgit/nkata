# Clock in and out

## Goal
Measure how long a task really takes.

## What to do
- On the task press `<prefix>xi` (clock in). Go elsewhere, press `<prefix>xj` to jump back.
- Press `<prefix>xo` (clock out). Open the `:LOGBOOK:` drawer and read your CLOCK line.
- Now make it the line in the final result: change both times with `ci[`
  to `2026-10-24 Sat 09:00` and `2026-10-24 Sat 09:45`.
- Press `<C-c><C-c>` on the CLOCK line. The duration fixes itself.

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- The real clock line has today's time, so the check uses fixed times.
- With the lualine snippet from the README, the statusline shows time against the Effort.
