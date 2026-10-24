agenda = {
  custom_commands = {
    w = {
      description = "Work overview",
      types = {
        { type = "agenda", span = "day", header = "Today" },
        { type = "tags_todo", match = "+work/!", header = "Open work tasks" },
        { type = "todo", match = "WAITING", header = "Waiting for" },
      },
    },
    u = { description = "Urgent", type = "tags", match = 'PRIORITY="A"|+urgent' },
  },
},
