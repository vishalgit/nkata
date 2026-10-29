# 03 Ask in plain words

You do not need brackets. The plain syntax is shorter:

`todo:NEXT tags:work !done`

- `todo:NEXT` means state NEXT
- `tags:work` means tagged work
- `!done` means "and not done"

## Goal

Find all tasks tagged **home** that are **not done**. Write the titles.

## Steps

1. Add this file to the agenda with `<C-c>[`.
2. Run `:Org ql_search`.
3. Type `tags:home !done`.
4. Under the last headline in `start.org`, type each matching title as a list
   item (`- Title`), in file order.

If your version complains about the query, look at `:h org-extensions-ql` for
the exact word for "not done".

## Done when

Your file matches `expected.org`.
