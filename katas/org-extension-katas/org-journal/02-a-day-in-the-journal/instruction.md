# 02 What a journal day looks like

The journal extension uses the org-journal format. A day is a level 1
headline with the date. Each entry is a level 2 headline that starts with a
time.

## Goal

Write two entries for Friday 30 October 2026.

| Time  | Title             | Text                      |
|-------|-------------------|---------------------------|
| 09:15 | Standup notes     | Talked about the release. |
| 14:30 | Called the dentist| Booked for Monday.        |

## Steps

1. Open `start.org`. The day headline is already there.
2. Go to the end of the headline and press `o`. Type `** 09:15 Standup notes`,
   press `<CR>`, then type the text.
3. Press `<CR>` and do the same for the second entry.

## See the plugin do it (not checked)

In your real journal press `<prefix>Lj` and type a title. The time is added for
you. Compare its shape to yours.

## Done when

Your file matches `expected.org`.
