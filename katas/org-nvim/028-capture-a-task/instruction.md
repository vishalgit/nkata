# Capture a task

## Goal
Jot down a task from anywhere without losing your place.

## What to do
- Press `<prefix>c`, choose `t` (Task). Type `Call the bank`. Finish with `<C-c><C-c>`.
- Open the notes file in a split: `:sp ~/org/refile.org`. Find your task.
- Yank only its headline line (`yy`), close the split, and paste it under `Captured`.
- Make it a child of `Captured` with `>>` or `<<` if the stars don't match.

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- Abort a capture with `<C-c><C-k>`.
- Select a line in Visual mode before `<prefix>c`: the selection gets quoted in the task.
