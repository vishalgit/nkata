# 02 One file, many posts

Hugo wants one markdown file per post. With the `EXPORT_FILE_NAME` property,
each subtree becomes its own post. You can keep every post in one org file.

## Goal

Name the two posts.

| Post                | EXPORT_FILE_NAME   |
|---------------------|--------------------|
| My first post       | `my-first-post`    |
| Learning org mode   | `learning-org-mode`|

## Steps

For each post headline:

1. Put the cursor on it.
2. Press `<prefix>p`. Choose `EXPORT_FILE_NAME`. Type the value.

A drawer appears right under the headline.

Do not export yet. That is for kata 04.

`<prefix>` means `<leader>o` (or the prefix you set).

## Done when

Your file matches `expected.org`.
