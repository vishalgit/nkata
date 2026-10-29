# shared/mod.rs

**File:** `crates/domain/src/shared/mod.rs` (part 1 of 2)  
**Save to:** `~/code/todo/crates/domain/src/shared/mod.rs`

## Goal
Declare `ids` privately and re-export what callers may use.

## Why
A private module plus `pub use` lets you move code between files later without changing a single import elsewhere.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/shared/mod.rs`.
