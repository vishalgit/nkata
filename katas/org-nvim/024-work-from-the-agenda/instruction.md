# Work from the agenda

## Goal
Change tasks without leaving the agenda.

| Key            | Action                       |
|----------------|------------------------------|
| `t`            | change the TODO state        |
| `s` `d`        | schedule / deadline          |
| `,` `+` `-`    | set / raise / lower priority |
| `:`            | set tags                     |
| `I` `O`        | clock in / out               |
| `<CR>` `<Tab>` | open the entry               |
| `F` `E` `q`    | follow mode / body text / quit |

## What to do
- `<C-c>[`, then `<prefix>a` `a`, then `gd` to 2026-10-26.
- Mark `Email the landlord` DONE with `t`.
- Reschedule `Dentist appointment` with `s` to `2026-10-29 15:00`.
- Raise `Review the pull request` with `+`.
- Tag `Buy groceries` with `:` and `home`.
- Quit with `q` and look at the editor.

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- Marking DONE adds a `CLOSED: [today]` stamp. The check can't know today's time, so remove it: `:%s/CLOSED: \[[^]]*\]\s*//` then `:g/^\s*$/d`.
- The check wants one space before the tags. If org.nvim pads tags out to a column, squeeze them back with `:%s/\s\+\(:\S\+:\)$/ \1/`.
- With Effort set, `_` then `<0:30` filters to short tasks.
