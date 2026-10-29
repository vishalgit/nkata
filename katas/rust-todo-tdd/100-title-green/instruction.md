# Green — the least that compiles and passes

**File:** `crates/domain/src/todos/title.rs` (part 2 of 15)  
**Save to:** `~/code/todo/crates/domain/src/todos/title.rs`

## Goal
A newtype with a private field, a smart constructor that accepts anything, and an error type with one variant.

## Why
- The private field is the whole point: outside this module nobody can write `TodoTitle(…)`, so every title in the program went through `parse`.
- `parse` accepting everything is deliberate. The next tests will force each rule in, one at a time.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/title.rs`.
