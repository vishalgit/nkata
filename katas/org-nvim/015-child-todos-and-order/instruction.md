# Child TODOs and order

## Goal
Progress counters from child tasks, and "do these in order".

## What to do
- Mark the three children of `Plan the trip` DONE, one by one. Watch the cookie.
- On `Ordered steps` press `<C-c><C-x>o`. An ORDERED property appears.

## Help
- Marking DONE adds a `CLOSED: [today]` stamp. The check can't know today's time, so remove it: `:%s/CLOSED: \[[^]]*\]\s*//` then `:g/^\s*$/d`.
- Blocking (you can't finish step two before step one) needs `enforce_todo_dependencies = true` in your setup.
- If the property is written with extra spaces, the check wants `:ORDERED: t`.
