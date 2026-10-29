# 02 Link to another note

Roam notes are normal org files. Each one has an `ID` and a `#+title:`.
You link notes by ID, so a link keeps working when you rename the file.

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
```

## Goal

Add a link to the note "Plain text habits" at the end of the "See also:" line.

## Steps

1. Open `start.org`. Go to the "See also:" line.
2. Press `A`, type one space, press `<Esc>`.
3. Press `<prefix>mi` (m = roam, i = insert).
4. Pick "Plain text habits" from the list.

The link looks like `[[id:...][Plain text habits]]`.

If you type a title that does not exist, roam makes a new note. Do not do that
here. Pick the existing one.

If the note is not in the list, restart Neovim once so roam can index
`~/roam`.

`<prefix>` means `<leader>o` (or the prefix you set).

## Done when

Your file matches `expected.org`.
