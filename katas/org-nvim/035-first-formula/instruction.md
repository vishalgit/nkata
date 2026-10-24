# Your first formula

## Goal
A table that does the maths.

## What to do
- Change Coffee's quantity from 3 to 2. Press `<C-c><C-c>` on the `#+TBLFM:` line.
- In the small table, type `=$1*2` in the first empty `double` field and press `<Tab>`.
  The formula moves to a new TBLFM line. Press `<C-c><C-c>` on it.

## Help
- `$4` is column 4. `@>` is the last row. `@I..@II` is the rows between the first and second line.
- `;%.2f` means two decimals.
