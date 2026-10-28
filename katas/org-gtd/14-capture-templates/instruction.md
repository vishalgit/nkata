# 14 Capture from anywhere

Capture templates let you add to your GTD file without opening it. You press
a key, type one line, and it lands in the right list.

## Goal

Make three templates:

- `i`: a new Inbox item with a time stamp
- `n`: a new next action (asks you for a tag)
- `w`: a waiting-for item (asks you for the follow-up date)

## Steps

Open `start.lua` and fill in the table so it matches this exactly (2 spaces
per indent):

```lua
return {
  capture = {
    templates = {
      i = { description = "Inbox", template = "* %?\n  %U", target = "~/org/gtd.org", headline = "Inbox" },
      n = { description = "Next action", template = "* NEXT %? %^g", target = "~/org/gtd.org", headline = "Next Actions" },
      w = { description = "Waiting for", template = "* WAITING %?\n  SCHEDULED: %^t", target = "~/org/gtd.org", headline = "Waiting For" },
    },
  },
}
```

## What do the odd words mean?

- `%?` is where the cursor starts.
- `%U` is the time now, as an inactive date.
- `%^g` asks you for tags. `%^t` opens the calendar for a date.
- `headline = "Inbox"` files the entry under that headline.

## How to use it for real

Copy the `capture = { ... }` part into your org.nvim options and restart.
From any buffer press `<prefix>c`, then `i`. Type your thought. Finish with
`<C-c><C-c>`. Abort with `<C-c><C-k>`.

## Done when

Your file matches `expected.lua`.
