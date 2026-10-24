# TDD green: the Button component

**File:** `src/components/button.rs` (part 2 of 2)
**Save to:** `~/code/design-kit/src/components/button.rs`

## Goal
Add `button_class` and the `Button` component.

## Why
- `#[prop(optional)]` uses the type's `Default`: no `variant` prop means `Primary`, no `size` means `Md`.
- `#[prop(optional, into)] disabled: Signal<bool>` accepts `true`, a signal, or a closure. Inside, read it with `.get()`.
- `on_click: Option<Callback<MouseEvent>>` with `into` lets the caller pass a plain closure.
- `disabled=move || …` and `aria-busy=move || …` are reactive attributes. `then_some("true")` gives `None` when false, and `None` removes the attribute.
- `<Show when=…>` mounts the spinner only while loading.

## Check
```sh
cargo test
```
**Green**: all tests pass, including the contract test.

## Help
