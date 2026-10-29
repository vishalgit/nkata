# Refactor access.rs to ask the policy

**File:** `crates/app/src/features/todos/access.rs` (part 3 of 3)  
**Save to:** `~/code/todo/crates/app/src/features/todos/access.rs`

## Goal
`load_for` takes the action in hand, as a function of the owner it is about to discover.

## Why
The caller knows *which* action; only `load_for` knows the owner. A closure `UserId -> Action` lets each say its part.

## Check
The four slices no longer compile: they call `load_for` with three arguments. The next four katas fix each call site.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/access.rs`.
