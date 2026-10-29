# 03 Aliases, tags and refs

A node can have other names (aliases), tags, and a web page it is about
(a ref). They live in the top drawer and in a `#+filetags:` line.

## Goal

Give "Org mode basics" two aliases, one ref and two tags.

| Thing   | Value                       |
|---------|-----------------------------|
| aliases | `Org mode` and `Org`        |
| ref     | `https://orgmode.org`       |
| tags    | `notes` and `emacs`         |

## Steps

1. Open `start.org`.
2. Inside the drawer, under `:ID:`, add the lines (one space after each colon
   name):

   ```
   :ROAM_ALIASES: "Org mode" Org
   :ROAM_REFS: https://orgmode.org
   ```

   A name with a space needs quotes. A one-word name does not.
3. Under `#+title:` add this line:

   `#+filetags: :notes:emacs:`

4. Save. Then use `<prefix>mf` and type `Org`. The note should be found by its
   alias too.

These names come from org-roam v2, so the same file works in Emacs.

`<prefix>` means `<leader>o` (or the prefix you set).

## Done when

Your file matches `expected.org`.
