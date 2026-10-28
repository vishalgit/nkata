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
