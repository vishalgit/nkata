# Authentication yields an actor

**File:** `crates/domain/src/identity/role.rs` (part 3 of 3)  
**Save to:** `~/code/todo/crates/domain/src/identity/role.rs`

**▶ Task**
```sh
scripts/backlog task TODO-15 6 domain "authenticate (6.11)" authenticate
```

## Goal
Add `Actor` to `role.rs`: who is asking, in what role.

## Why
Authentication's output is not "a user row"; it is the two facts every later decision needs. Chapter 10's policy takes an `Actor`.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/role.rs`.
