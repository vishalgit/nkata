# 04 Tags, categories and drafts

Three things in the headline become front matter:

- plain tags become Hugo **tags**
- a tag that starts with `@` becomes a Hugo **category**
- a post with a TODO word is a **draft**

## Goal

| Post               | State | Tags                    |
|--------------------|-------|-------------------------|
| My first post      | none  | `orgmode`, `@guides`    |
| Learning org mode  | TODO  | `orgmode`, `tips`, `@guides` |

## Steps

1. Open `start.org`.
2. On "My first post" press `<prefix>t`. Choose `orgmode` and `@guides`.
   (Use `<Tab>` in the menu to type a tag by hand.)
3. On "Learning org mode" add the three tags the same way.
4. On "Learning org mode" press `cit` once to make it `TODO`.

## Try it for real (not checked)

Press `<leader>oe`, then `H` in the export menu. Then read the files:

```sh
cat ~/site/content/posts/my-first-post.md
cat ~/site/content/posts/learning-org-mode.md
```

Look at the front matter at the top. The second post should say it is a draft.

Spaces: the expected file uses one space before tags and one space in property lines (`:Effort: 4:00`). Set `tags_column = 0` if your tags get pushed to the right.

## Done when

Your file matches `expected.org`.
