# Match syntax

## Goal
Ask the agenda sharp questions.

## What to do
- `<C-c>[`, then `<prefix>a` then `m`, and type each match from the Answers list.
- After `::` write the headline names it found, in file order, joined by `, `.
  (Names only: no keyword, priority or tags.)

| Match          | Finds                          |
|----------------|--------------------------------|
| `+work-urgent` | tagged work but not urgent     |
| `+work/NEXT`   | work entries in state NEXT     |
| `+work/!`      | work entries that are not done |
| `PRIORITY="A"` | priority A                     |
| `home`         | tagged home                    |

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- More to try: `LEVEL=2`, `work|home`, `SCHEDULED<="<+2d>"`, `Effort<"1:00"`.
