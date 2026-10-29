# 03 Plan a future day

Future days can hold scheduled entries. The agenda shows them on that day, so
the journal doubles as a calendar.

## Goal

On Monday 2 November 2026 add a TODO that is scheduled at 15:00.

## Steps

1. Open `start.org`. The day headline is already there.
2. Under it add the headline `** TODO Dentist appointment`.
3. On that headline press `<prefix>s`, then `i`, and type
   `2026-11-02 15:00`, then `<CR>`.

The SCHEDULED line goes right under the headline.

## Check it (not checked)

Add the file to the agenda with `<C-c>[`. Go to the day with `gd` and
`2026-11-02`. The appointment should show at 15:00.

## Done when

Your file matches `expected.org`.
