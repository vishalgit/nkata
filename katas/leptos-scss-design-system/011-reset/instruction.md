# vendors/_reset

**File:** `style/vendors/_reset.scss`
**Save to:** `~/code/design-kit/style/vendors/_reset.scss`

## Goal
Type a small reset into `vendors/`.

## Why
- `vendors/` holds CSS you didn't write and don't edit. If you need to change a vendor rule, override it later in `base/` so a vendor upgrade doesn't wipe your change.
- `box-sizing: border-box` everywhere makes `width` include padding and border.
- `input, button { font: inherit; }` stops form controls from using the browser's own font.
- The `[hidden]` rule matters later: the Alert sets `display: grid`, which would otherwise beat the browser's own `[hidden] { display: none }` and the dismiss button would do nothing.
- Reduced motion: if the OS asks for less motion, transitions and the spinner go nearly instant.

## Check
Nothing loads it yet.

## Help
