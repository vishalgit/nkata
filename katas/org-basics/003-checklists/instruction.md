# Checklists and progress cookies

## Goal
The editor has a packing list as bare lines. Turn it into an org checklist,
tick off what's packed, and add a progress cookie to the heading.

## What to do
- Each item becomes a checkbox list item.
- `Charger` is packed (ticked); `Passport` and `Toothbrush` are not.
- The heading shows progress: 1 of 3 done.

## Help
- An unticked item: `- [ ] Passport`. A ticked one: `- [X] Charger`.
- A progress cookie sits at the end of the heading: `* Packing list [1/3]`.
- org.nvim may tick boxes and update the cookie for you with a mapping;
  typing it by hand passes just the same.
- Try a visual-block insert (`<C-v>` then `I`) to prefix all three lines at once.
