# 04 Filter the board

A big board is noisy. Press `/` to show only the cards you care about. The
filter is a tag match, like in the agenda.

## Goal

Move every **work** card one column to the right. Do not touch the home cards.

## Steps

1. Open `start.org` and run `:Org kanban`.
2. Press `/` and type `+work`, then `<CR>`. Only work cards remain.
3. Put the cursor on each visible card and press `l` once.
4. Press `/` and clear the filter if you want to check the home cards.
   They must not have moved.
5. Press `q` and save with `:w`.

Spaces: the expected file uses one space before tags and one space in property lines (`:Effort: 4:00`). Set `tags_column = 0` if your tags get pushed to the right.

## Done when

Your file matches `expected.org`.
