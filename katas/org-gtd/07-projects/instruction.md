# 07 Make a project

In GTD, a project is any goal that needs more than one step. A project is a
headline. Its steps are children. Every project must have at least one `NEXT`.

## Goal

Turn `Plan the family trip` into a project with three steps and a counter.

## Steps

1. Open `start.org`. The cursor goes on `** Plan the family trip`.
2. Press `<prefix>is` to insert a sub-headline. Type `NEXT Decide the dates`.
3. Press `<M-CR>` for the next sibling. Type `TODO Book the flights`.
4. Press `<M-CR>` again. Type `TODO Reserve the hotel`.
5. Go back to `Plan the family trip`. Go to the end of the line and type
   ` [/]`. Press `<prefix>#`. The counter turns into `[0/3]`.
6. Still on that headline press `<prefix>t`, then `j` for the `project` tag.

## Why the tag?

The `project` tag lets the agenda find your projects (kata 12).

## Note on spaces

The tag goes at the end of the line. The expected file assumes one space before
the tag (`tags_column = 0`). If your result only differs in spaces before the
tag, that is why.

## Done when

Your file matches `expected.org`.
