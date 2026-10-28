# 12 Find stuck projects

A project is **stuck** when it has no `NEXT` (or `TODO`) step. It will never
move, because there is nothing to do. Finding these is the best part of the
weekly review.

## Goal

Find the stuck project and give it a next action:
`NEXT Install rustup`.

## Steps

1. Press `<prefix>a`, then `#`. The agenda lists stuck projects.
   (If the agenda cannot see this file, press `<C-c>[` first.)
2. You should see `Learn Rust`. Close the agenda with `q`.
3. Put the cursor on `** Learn Rust :project:`.
4. Press `<prefix>is` to insert a sub-headline.
5. Type `NEXT Install rustup`.

## Why only one?

`Repaint the kitchen` has a `NEXT`. `Plan the family trip` has a `TODO`.
Both are fine.

## Note

If children also show up as stuck, they inherit the `project` tag. See
`:h org-config` for `tags_exclude_from_inheritance` and add `"project"`.
You only need that for the agenda, not for this file.

## Done when

Your file matches `expected.org`.
