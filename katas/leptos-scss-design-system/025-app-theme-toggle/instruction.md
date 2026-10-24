# Theme as a signal

**File:** `src/app.rs` (part 2 of 3)
**Save to:** `~/code/design-kit/src/app.rs`

## Goal
Wrap the page in `.app`, add the header, and toggle `data-theme` with a signal.

## Why
- `Theme` is an enum, not a bool. `as_str()` returns exactly what the SCSS selector matches.
- `RwSignal::new(Theme::default())` holds reactive state. `theme.update(|t| *t = t.toggled())` changes it.
- `data-theme=move || theme.get().as_str()` is a reactive attribute: Leptos re-runs the closure and updates only that attribute when the signal changes.
- The button is a plain `<button>` for now. The Button component replaces it in Part G.
- The tests pin the Rust side of the contract: `Theme::Dark.as_str()` must be `"dark"`, because `themes/_dark.scss` says `[data-theme='dark']`.

## Check
```sh
cargo test
```
In the browser, click **Toggle theme**: the page goes dark.

## Help
