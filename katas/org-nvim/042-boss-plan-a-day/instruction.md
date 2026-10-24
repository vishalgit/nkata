# Boss kata: plan a day

## Goal
Use many skills together, from a blank file, without looking at earlier katas.

## What to do
- Add the `#+TODO:` line. Make `Launch the side project`: TODO, priority A, tag `project`, a `[/]` cookie.
- Under it make `Write the landing page`, `Record the demo` (deadline 2026-11-06),
  `Plan the launch` (scheduled 2026-11-02 10:00, Effort 2:00). All TODO.
- Give `Plan the launch` a 4-item checklist with a cookie. Tick two.
- Capture `Post a teaser` with `<prefix>c` `t`. `<C-c>[`, then refile it under the project.
- From the agenda (`<prefix>a` `t`), mark `Write the landing page` DONE.
- Add a `Budget` table that sums three costs with a formula.
- Export to Markdown with `<prefix>e`.

## Help
- `<prefix>` means `<leader>o`. `u` undoes, `<leader>kr` resets the kata.
- Marking DONE adds a `CLOSED: [today]` stamp. The check can't know today's time, so remove it: `:%s/CLOSED: \[[^]]*\]\s*//` then `:g/^\s*$/d`.
- The capture template adds a timestamp and link under the task. Delete those lines.
- Win: everything matches, and you opened `g?` no more than twice.
