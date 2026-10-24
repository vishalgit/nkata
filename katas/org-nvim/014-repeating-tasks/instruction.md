# Repeating tasks

## Goal
Tasks that come back by themselves.

| Repeater | Meaning                                    |
|----------|--------------------------------------------|
| `+1w`    | move one week forward, once                |
| `++1w`   | keep adding weeks until the date is future |
| `.+2d`   | move to two days after today               |

## What to do
- Mark each task DONE with `cit`. Watch the date jump and the state go back to TODO.

## Help
- The check uses `+` repeaters only, because `++` and `.+` depend on today.
  Try them after: change one to `.+3d`, mark it DONE, look, then `<leader>kr`.
- org.nvim may log the repeat (a `LAST_REPEAT` property or a `- State` line). Remove it with
  `:g/^\s*\(:PROPERTIES:\|:LAST_REPEAT:\|:LOGBOOK:\|:END:\|- State\)/d`
