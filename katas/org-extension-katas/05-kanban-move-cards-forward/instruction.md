# 02 Move cards forward

A kanban board has one column per TODO word. Each task is a card. Moving a
card changes the TODO word in your file.

## Goal

| Card              | From   | To        |
|-------------------|--------|-----------|
| Write the report  | TODO   | NEXT      |
| Call the dentist  | NEXT   | WAITING   |
| Pay the bill      | TODO   | stays     |

## Steps

1. Open `start.org` and run `:Org kanban`.
2. Move the cursor onto the card "Write the report".
3. Press `l` once. The card moves one column to the right.
4. Move onto "Call the dentist" and press `l` once.
5. Leave "Pay the bill" alone.
6. Press `q` to close the board. Save the file with `:w`.

The words go in this order: TODO, NEXT, WAITING, SOMEDAY, then the done words.
Do not push anything into DONE in this kata.

`<prefix>` means `<leader>o` (or the prefix you set).

## Done when

Your file matches `expected.org`.
