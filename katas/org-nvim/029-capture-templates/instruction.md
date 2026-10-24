# Capture templates

## Goal
Make your own capture menu.

## What to do
- Type the `capture` config shown in the final result pane.
- Put it inside your `require("org").setup({ ... })` table. Restart `kvim`.
- `<prefix>c` then try `wt` (work task), `j` (journal) and `l` (log line).
- Check that the journal entry sits under today's date in `~/org/journal.org`.

## Shell
```sh
touch ~/org/work.org ~/org/journal.org
printf '* Inbox\n' >> ~/org/work.org
printf '* Expenses\n| When | Amount | What |\n' > ~/org/log.org
```

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- Escapes: `%?` cursor, `%U` inactive timestamp, `%a` link back, `%i` selection, `%^{Prompt}` ask for text.
