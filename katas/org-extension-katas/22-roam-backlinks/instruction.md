# 04 Backlinks: who links to me?

A backlink is a note that links to this one. Roam shows them in a side window,
so you can walk your notes like a web.

## Setup (shell, run once)

```sh
mkdir -p ~/roam
cat > ~/roam/plain-text-habits.org <<'EOF'
:PROPERTIES:
:ID:       4f1c2a10-0002-4a00-8000-000000000002
:END:
#+title: Plain text habits

Plain text lasts longer than any app.
EOF
cat > ~/roam/org-mode-basics.org <<'EOF'
:PROPERTIES:
:ID:       4f1c2a10-0001-4a00-8000-000000000001
:END:
#+title: Org mode basics

See also: [[id:4f1c2a10-0002-4a00-8000-000000000002][Plain text habits]]
EOF
cat > ~/roam/reading-list.org <<'EOF'
:PROPERTIES:
:ID:       4f1c2a10-0003-4a00-8000-000000000003
:END:
#+title: Reading list

Start here: [[id:4f1c2a10-0002-4a00-8000-000000000002][Plain text habits]]
EOF
```

## Goal

Find every note that links to "Plain text habits".

## Steps

1. Open `~/roam/plain-text-habits.org` in a second window.
2. Press `<prefix>ml` to open the backlinks window.
3. Read the note titles in it.
4. Come back to `start.org`. Type the titles as list items under the headline,
   in alphabetical order.

If the list is empty, restart Neovim once so roam can index `~/roam`.

`<prefix>` means `<leader>o` (or the prefix you set).

## Done when

Your file matches `expected.org`.
