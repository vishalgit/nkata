# 03 Settings you write once

Properties that start with `EXPORT_HUGO_` are inherited. Put them on the
parent headline and every post below uses them.

## Goal

On the headline `Blog`, set:

| Property                | Value    |
|-------------------------|----------|
| `EXPORT_HUGO_BASE_DIR`  | `~/site` |
| `EXPORT_HUGO_SECTION`   | `posts`  |

## Steps

1. Open `start.org`. Put the cursor on `* Blog`.
2. Press `<prefix>p` twice, once for each property.
3. The `Blog` drawer now holds two lines. The posts below keep their own
   `EXPORT_FILE_NAME`.

Base dir is your Hugo site. Section is the folder under `content/`. So the
posts land in `~/site/content/posts/`.

These names come from ox-hugo. Check `:h org-extensions-hugo` if one is
refused.

## Done when

Your file matches `expected.org`.
