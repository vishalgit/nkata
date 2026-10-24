# TextField styles

**File:** `style/components/_field.scss`
**Save to:** `~/code/design-kit/style/components/_field.scss`

## Goal
Style the text field.

## Why
- Two blocks in one partial: `.field` (label, hint, error) and `.input` (the control). They're separate because an input can appear without the field wrapper.
- `&__error:empty { display: none; }` hides the error paragraph when it has no text.
- `--invalid` swaps the border and the focus glow to danger tokens.
- The file is `_field.scss` while the Rust file is `text_field.rs`: the partial is named after its main BEM block.

## Check
Nothing loads it yet.

## Help
