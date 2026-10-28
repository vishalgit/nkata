# 10 Dates: scheduled and deadline

Two kinds of dates, two meanings:

- **SCHEDULED**: "I plan to start on this day."
- **DEADLINE**: "It must be done by this day."

GTD says: use dates only for things that really have a date. Do not date
everything.

## Goal

| Item                  | Date                               |
|-----------------------|------------------------------------|
| Start the report      | scheduled `2026-11-02`             |
| Submit the tax return | deadline `2026-11-30`, warn 7 days before |
| Renew the passport    | deadline `2026-12-15`, warn 14 days before |

## Steps

1. On `Start the report` press `<prefix>s`, press `i`, type `2026-11-02`,
   press `<CR>`.
2. On `Submit the tax return` press `<prefix>d`, press `i`, type
   `2026-11-30`, press `<CR>`.
3. The warning is a small word inside the date. Put the cursor right before
   the `>` and type ` -7d` so it reads `<2026-11-30 Mon -7d>`.
4. Do the same for the passport: `2026-12-15`, then ` -14d`.

## What does `-7d` do?

The agenda starts showing the deadline 7 days before it is due.

## Done when

Your file matches `expected.org`.
