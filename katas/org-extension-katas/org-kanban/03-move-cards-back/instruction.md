# 03 Move cards back

`h` moves a card to the column on the left. Same rule: only the TODO word
changes.

## Goal

| Card                       | From     | To      |
|----------------------------|----------|---------|
| Reply from the landlord    | WAITING  | NEXT    |
| Learn the piano            | SOMEDAY  | NEXT    |
| Fix the squeaky door       | NEXT     | TODO    |

## Steps

1. Open `start.org` and run `:Org kanban`.
2. On "Reply from the landlord" press `h` once.
3. On "Learn the piano" press `h` two times. (It passes through WAITING.)
4. On "Fix the squeaky door" press `h` once.
5. Press `q` and save with `:w`.

## Done when

Your file matches `expected.org`.
