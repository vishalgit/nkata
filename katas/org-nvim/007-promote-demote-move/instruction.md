# Promote, demote, move

## Goal
Reshape an outline.

| Key             | Does                                         |
|-----------------|----------------------------------------------|
| `<<` `>>`       | promote / demote the headline (or list item) |
| `<s` `>s`       | promote / demote the whole subtree           |
| `<M-h>` `<M-l>` | promote / demote (also in Visual mode)       |
| `<M-k>` `<M-j>` | move subtree up / down                       |

## What to do
- Put the steps in order with `<M-j>` and `<M-k>`: 1, 2, 2b, 3.
- Make `Step 2b` a child of `Step 2` with `>>`.

## Help
- Moving `Step 2` now carries `Step 2b` with it. Try it, then put it back.
