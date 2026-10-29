# Green — ErrorMessage

**File:** `crates/app/src/ui/error_message.rs` (part 2 of 3)  
**Save to:** `~/code/todo/crates/app/src/ui/error_message.rs`

## Goal
A component that renders an alert when its signal holds an error.

## Why
- The class names are BEM: block `alert`, modifier `alert--error`. The tests check them because the 7-1 SCSS later in this chapter styles exactly these.
- `role="alert"` makes screen readers announce it — and gives Playwright a selector (`getByRole('alert')`).

## Help
- Save with `:w! ~/code/todo/crates/app/src/ui/error_message.rs`.
