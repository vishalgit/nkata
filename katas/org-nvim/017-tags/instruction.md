# Tags

## Goal
Label headlines.

## What to do
- On `Book a room` press `<prefix>t` then `r`. `@remote` replaces `@office` (one exclusive group).
- Select `Send the agenda` and `Order lunch` with `V`, press `<prefix>t` and pick `u`. Both get `urgent`.
- On `Plan the offsite` add `p` (project) next to `work`.

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- The check wants one space before the tags. If org.nvim pads tags out to a column, squeeze them back with `:%s/\s\+\(:\S\+:\)$/ \1/`.
- Children inherit tags: `Order lunch` is also `work`, without writing it.
- `:Org align_tags` lines all tags up in one column. Try it, look, then `u`.
