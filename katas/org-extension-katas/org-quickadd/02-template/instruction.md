# 02 A capture template that understands sentences

Capture templates can use the quick add syntax too. You turn it on with
`quickadd = true` inside the template.

## Goal

Make a template called `q` that files quick add lines under `Inbox`.

## Steps

1. Open `start.lua`.
2. Inside `templates = { }` add this one line (6 spaces of indent):

```lua
      q = { description = "Quick add", quickadd = true, target = "~/org/gtd.org", headline = "Inbox" },
```

3. Copy it into your real options and restart.
4. Press `<prefix>c`, then `q`. Type `Buy milk #home` and finish the capture.

Check `:h org-extensions-quickadd` if the template needs more fields on your
version.

## Done when

Your file matches `expected.lua`.
