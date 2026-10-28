# 08 Put a context on every next action

A next action is only useful if you know where you can do it.

## Goal

Give each next action one context tag.

| Next action                 | Tag          |
|-----------------------------|--------------|
| Call the dentist            | `@phone`     |
| Buy printer paper           | `@errands`   |
| Pay the electricity bill    | `@computer`  |
| Fix the squeaky door        | `@home`      |

## Steps

1. Put the cursor on a headline.
2. Press `<prefix>t`. A menu opens.
3. Press the fast key: `p` phone, `e` errands, `c` computer, `h` home.
4. Do it for all four.

## Note on spaces

The expected file assumes one space before the tag (`tags_column = 0`).

## Done when

Your file matches `expected.org`.
