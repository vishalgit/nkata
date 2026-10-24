capture = {
  templates = {
    t = { description = "Task", template = "* TODO %?\n  %U\n  %a", target = "~/org/refile.org" },
    w = "Work",
    wt = { description = "Work task", template = "* TODO %? :work:", target = "~/org/work.org", headline = "Inbox" },
    j = { description = "Journal", template = "* %<%H:%M> %?", target = "~/org/journal.org", datetree = true },
    l = { description = "Log line", type = "table-line", template = "| %U | %^{Amount} | %^{What} |",
          target = "~/org/log.org", headline = "Expenses", immediate_finish = true },
  },
},
