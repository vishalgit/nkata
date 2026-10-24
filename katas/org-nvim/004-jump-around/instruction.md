# Jump around

## Goal
Move between headlines without scrolling.

| Key             | Goes to                          |
|-----------------|----------------------------------|
| `]]` `[[`       | next / previous visible headline |
| `][` `[]`       | next / previous sibling          |
| `g{`            | parent headline                  |
| `<prefix>.`     | pick any headline from a list    |

## What to do
Start on line 1. After each jump, add the word shown to the end of the headline (`A` then type).
1. `]]` three times lands on `Deep child`. Add ` - ok`.
2. `g{` goes to its parent. Add ` - parent`.
3. `[]` goes to the previous sibling. Add ` - prev`.
4. `][` twice goes to the last sibling. Add ` - next`.
5. `<prefix>.` and pick `Far away`. Add ` - picked`.

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- Counts work: `3]]` jumps three headlines at once.
