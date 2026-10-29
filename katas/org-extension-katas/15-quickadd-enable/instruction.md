# 01 Turn on the quickadd extension

Extensions are off until you ask for them. You ask in `setup()`, under
`extensions`.

## Goal

Add `quickadd` to the `extensions` table.

## Steps

1. Open `start.lua`.
2. Between the two braces of `extensions = { }` add this line (4 spaces of
   indent):

   `quickadd = true,`

3. Copy the same line into your real org.nvim options, restart Neovim and run
   `:checkhealth org`. It lists enabled extensions and what each one needs.

Try it: press `<prefix>q`, type `Buy milk`, and look at the live preview.

## What happens if I leave it out?

Nothing breaks. The extension just stays unloaded.

Stuck? Run `:h org-extensions-quickadd`. Keys and options can change, because this extension is stable.

## Done when

Your file matches `expected.lua`.
