# Export

## Goal
Turn notes into a document.

## What to do
- Add the three header lines at the top (shown in the final result).
- Press `<prefix>e` and export to Markdown, then HTML. Open both.

## Shell
LaTeX, PDF, DOCX and ODT need pandoc (and latexmk or pdflatex for native PDF):
```sh
pandoc --version | head -1
command -v latexmk pdflatex
```

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- `toc:nil` drops the table of contents, `num:nil` drops section numbers.
