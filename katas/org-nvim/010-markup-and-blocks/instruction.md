# Markup and blocks

## Goal
Style text and add blocks.

## What to do
- On the empty line under `Type the styled sentence here:` type:
  `*bold* /italic/ _underline_ =verbatim= ~code~ +strike+`
- Select the word `important` in Visual mode and press `<prefix>E`. Pick bold.
- Go to the end (`G`), add an empty line, then press `<prefix>ib` and pick quote. Type `Plain text lasts forever.`
- Add another empty line, then `<prefix>ib` again for an example block: `Shown exactly as typed.`

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- The check wants lowercase `#+begin_quote`. If the menu writes uppercase, fix it with `gu` on the line.
