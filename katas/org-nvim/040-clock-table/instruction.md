# Clock table

## Goal
A report of where your time went.

## What to do
- On the `#+BEGIN:` line press `<C-c><C-c>`. The table fills in.
- Change `:maxlevel 2` to `:maxlevel 3` and update again. Try `:tags t` too, then take it out.
- Fill in the answers.
- Last, empty the report: delete the lines between `#+BEGIN:` and `#+END:`
  (the table has a timestamp in it, so it can't be checked).

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- `<prefix>xr` inserts a fresh clock table anywhere.
- `:block thisweek` limits the report to this week.
