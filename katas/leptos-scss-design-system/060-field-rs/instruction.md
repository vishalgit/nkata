# The TextField component

**File:** `src/components/text_field.rs`
**Save to:** `~/code/design-kit/src/components/text_field.rs`

## Goal
Write the `TextField` component with its tests.

## Why
- `value: RwSignal<String>` is two-way binding: `prop:value` writes into the input, `on:input:target` writes back.
- `on:input:target` types the event target as the input element, so `ev.target().value()` needs no cast.
- `class:input--invalid=invalid` adds the modifier class only while the error signal holds `Some`.
- `aria-describedby` points the input at its hint and error, so a screen reader reads them with the label.

## Check
Needs the export in `mod.rs` (next kata).

## Help
