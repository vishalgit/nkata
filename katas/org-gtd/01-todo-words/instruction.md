# 01 Set up your TODO words

GTD needs more than TODO and DONE. You also need a word for "do this next",
one for "I am waiting for someone", and one for "maybe later".

## Goal

Tell org.nvim your GTD words.

## Steps

1. Open `start.org`. Go to the last line.
2. Press `o` and type this line exactly:

   `#+TODO: TODO(t) NEXT(n) WAITING(w) SOMEDAY(s) | DONE(d) CANCELLED(c)`

3. Press `<Esc>`. With the cursor on that line press `<C-c><C-c>`.
   This reloads the words.

## How to read the line

- Words before `|` are open. Words after `|` are finished.
- The letter in brackets is the fast key. `NEXT(n)` means press `n` in the
  state menu (`<prefix>T`).

## Done when

Your file matches `expected.org`.
