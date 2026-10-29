# Green — a variant, and the compiler insists on arms

**File:** `crates/domain/src/identity/policy.rs` (part 4 of 6)  
**Save to:** `~/code/todo/crates/domain/src/identity/policy.rs`

## Goal
Add `DeleteTodo` and its two arms.

## Why
Add the variant first and the compiler refuses the build until the `match` covers it: the policy can never forget an action.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/policy.rs`.
