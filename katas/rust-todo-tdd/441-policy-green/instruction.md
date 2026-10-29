# Green — an Action enum and one match

**File:** `crates/domain/src/identity/policy.rs` (part 2 of 6)  
**Save to:** `~/code/todo/crates/domain/src/identity/policy.rs`

## Goal
The action carries the facts the decision needs; `authorize` matches on (role, action).

## Why
- The policy is pure: it never loads anything. The shell loads the todo, then asks "may this actor do this action to a todo owned by X?".
- Matching on the **pair** `(role, action)` makes every combination explicit.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/policy.rs`.
