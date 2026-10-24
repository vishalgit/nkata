# The Alert component

**File:** `src/components/alert.rs`
**Save to:** `~/code/design-kit/src/components/alert.rs`

## Goal
Write the `Alert` component with its tests.

## Why
- `role()` maps tone to ARIA: `alert` interrupts a screen reader, `status` waits. Only warning and danger should interrupt.
- `hidden=move || !open.get()` hides the alert on dismiss. It works because of the `[hidden]` rule in the reset; without it, `display: grid` would keep the alert visible.
- `open` is local state: each Alert owns its own signal.

## Check
Needs the export in `mod.rs` (next kata).

## Help
