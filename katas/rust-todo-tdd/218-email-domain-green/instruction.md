# Green — one readable condition

**File:** `crates/domain/src/identity/email.rs` (part 6 of 8)  
**Save to:** `~/code/todo/crates/domain/src/identity/email.rs`

## Goal
Replace the `if` with one named condition.

## Why
An empty domain is caught too: it contains no `.`. Naming the whole condition `valid` keeps the rule readable as a list.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/email.rs`.
