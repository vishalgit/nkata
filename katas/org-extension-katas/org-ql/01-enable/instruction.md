# 01 Turn on the ql extension

Extensions are off until you ask for them. You ask in `setup()`, under
`extensions`.

## Goal

Add `ql` to the `extensions` table.

## Steps

1. Open `start.lua`.
2. Between the two braces of `extensions = { }` add this line (4 spaces of
   indent):

   `ql = true,`

3. Copy the same line into your real org.nvim options, restart Neovim and run
   `:checkhealth org`. It lists enabled extensions and what each one needs.

Try it: run `:Org ql_search`. You type a query and get a list of matching headlines.

## What happens if I leave it out?

Nothing breaks. The extension just stays unloaded.

Stuck? Run `:h org-extensions-ql`. Keys and options can change, because this extension is stable.

## Done when

Your file matches `expected.lua`.
