# AuthPage — registration first

**File:** `end2end/tests/pages/auth-page.ts` (part 1 of 2)  
**Save to:** `~/code/todo/end2end/tests/pages/auth-page.ts`

## Goal
The auth page object, with the one method the first journey needs.

## Why
Role- and label-based selectors (`getByLabel`, `getByRole`) double as an accessibility check: if a test cannot find the field by its label, neither can a screen reader.

## Help
- Save with `:w! ~/code/todo/end2end/tests/pages/auth-page.ts`.
