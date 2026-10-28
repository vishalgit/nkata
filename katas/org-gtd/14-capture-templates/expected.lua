return {
  capture = {
    templates = {
      i = { description = "Inbox", template = "* %?\n  %U", target = "~/org/gtd.org", headline = "Inbox" },
      n = { description = "Next action", template = "* NEXT %? %^g", target = "~/org/gtd.org", headline = "Next Actions" },
      w = { description = "Waiting for", template = "* WAITING %?\n  SCHEDULED: %^t", target = "~/org/gtd.org", headline = "Waiting For" },
    },
  },
}
