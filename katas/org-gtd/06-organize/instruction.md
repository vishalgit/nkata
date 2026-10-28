# 06 Organize (step 3 of GTD)

Move each clarified item out of the Inbox to its own list. An empty Inbox is
the goal.

## Goal

| Item                                      | Goes to          |
|-------------------------------------------|------------------|
| `NEXT Call the dentist`                   | Next Actions     |
| `WAITING Reply from the landlord ...`     | Waiting For      |
| `SOMEDAY Write a blog post ...`           | Someday/Maybe    |

## Steps

1. Put the cursor on `** NEXT Call the dentist`.
2. Press `<prefix>hd`. This cuts the whole subtree.
3. Go to `** NEXT Pay the electricity bill` (the only item in Next Actions).
4. Press `<prefix>hp`. It pastes the item after the one under the cursor, at
   the right level.
5. Do the same for the other two items.

## Keys

- `<prefix>hy` copy a subtree
- `<prefix>hd` cut a subtree
- `<prefix>hp` paste a subtree after the current one

## Bonus (not checked)

`<prefix>r` refiles a subtree to a place you pick. It needs this file to be in
your `agenda_files` (or press `<C-c>[` to add it for this session).

## Done when

Your file matches `expected.org`. The Inbox has no children.
