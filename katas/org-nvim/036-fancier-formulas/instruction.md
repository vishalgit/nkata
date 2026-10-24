# Fancier formulas

## Goal
Functions, durations, and tables that read other tables.

## What to do
- Press `<C-c><C-c>` on each `#+TBLFM:` line. Read what each column did.
- Change Coding's end time from `12:15` to `12:30` and update its table again.

## Help
- Functions: `vsum` `vmean` `vmin` `vmax` `vcount` `abs` `sqrt` `round` `mod` `if`.
- Anything inside `'( ... )` runs as Lua.
- `;U` writes hours and minutes.
- `remote(name, ref)` reads a table that has a `#+NAME:` line.
