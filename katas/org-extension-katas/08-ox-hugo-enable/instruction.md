# 01 Turn on the hugo extension

Extensions are off until you ask for them. You ask in `setup()`, under
`extensions`.

## Goal

Add `hugo` to the `extensions` table.

## Steps

1. Open `start.lua`.
2. Between the two braces of `extensions = { }` add this line (4 spaces of
   indent):

   `hugo = true,`

3. Copy the same line into your real org.nvim options, restart Neovim and run
   `:checkhealth org`. It lists enabled extensions and what each one needs.

Run `mkdir -p ~/site` too. You will export into it in the next katas.

## What happens if I leave it out?

Nothing breaks. The extension just stays unloaded.

Stuck? Run `:h org-extensions-hugo`. Keys and options can change, because this extension is experimental.

## Done when

Your file matches `expected.lua`.
