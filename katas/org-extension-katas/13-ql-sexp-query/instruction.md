# 02 Ask with an s-expression

org-ql lets you ask questions about your notes. One way is an s-expression:
a bracket, a word, then the parts.

`(and (todo "NEXT") (tags "work"))`

Read it as: "NEXT, and tagged work".

## Goal

Run the query on the tasks in this file, and write the matching titles.

## Steps

1. Add this file to the agenda for this session with `<C-c>[`.
2. Run `:Org ql_search`.
3. Type `(and (todo "NEXT") (tags "work"))`.
4. Under the last headline in `start.org`, type each matching title as a list
   item (`- Title`), in file order.

## Done when

Your file matches `expected.org`.
