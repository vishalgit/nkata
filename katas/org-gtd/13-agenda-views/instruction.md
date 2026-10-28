# 13 Engage: your GTD agenda (step 5 of GTD)

Engage means: look at the right list and do the work. One agenda page that
shows today, your next actions, what you wait for and stuck projects does
this job.

## Goal

Add two custom agenda commands to a config table.

- `g`: a GTD page with four blocks
- `h`: only `NEXT` actions tagged `@home`

## Steps

Open `start.lua` and fill in the table so it matches this exactly (2 spaces
per indent):

```lua
return {
  agenda = {
    stuck_projects = { match = "+project/-DONE" },
    custom_commands = {
      g = {
        description = "GTD overview",
        types = {
          { type = "agenda", span = "day", header = "Today" },
          { type = "todo", match = "NEXT", header = "Next actions" },
          { type = "todo", match = "WAITING", header = "Waiting for" },
          { type = "stuck", header = "Stuck projects" },
        },
      },
      h = { description = "Next at home", type = "tags_todo", match = "+@home/NEXT" },
    },
  },
}
```

## How to use it for real

Copy the `agenda = { ... }` part into your `require("org").setup({ ... })`
options. Restart Neovim. Press `<prefix>a`, then `g`. Press `<prefix>a`,
then `h` at home.

## Done when

Your file matches `expected.lua`.
