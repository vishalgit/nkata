# 01 Turn on the journal extension

Extensions are off until you ask for them. You ask in `setup()`, under
`extensions`.

## Goal

Add `journal` to the `extensions` table.

## Steps

1. Open `start.lua`.
2. Between the two braces of `extensions = { }` add this line (4 spaces of
   indent):

   `journal = true,`

3. Copy the same line into your real org.nvim options, restart Neovim and run
   `:checkhealth org`. It lists enabled extensions and what each one needs.

Try it: `<prefix>Lj` adds a timed entry to today. `<prefix>Ln` and `<prefix>Lp` go to the next and previous day. `<prefix>Lc` shows a calendar.

## What happens if I leave it out?

Nothing breaks. The extension just stays unloaded.

Stuck? Run `:h org-extensions-journal`. Keys and options can change, because this extension is experimental.

## Done when

Your file matches `expected.lua`.
