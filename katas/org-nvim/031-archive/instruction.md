# Archive

## Goal
Tidy away finished things.

## What to do
- On `An old task` press `<prefix>$`. It moves to an archive file and remembers where it came from.
- On `Old notes` press `<prefix>hA`. It stays, gets the ARCHIVE tag, and hides from the agenda.

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- The check wants one space before the tags. If org.nvim pads tags out to a column, squeeze them back with `:%s/\s\+\(:\S\+:\)$/ \1/`.
- Peek at the archive file with `:e %_archive` (then `<C-o>` back).
