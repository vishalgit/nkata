# Green — FromStr and Display

**File:** `crates/domain/src/shared/ids.rs` (part 4 of 6)  
**Save to:** `~/code/todo/crates/domain/src/shared/ids.rs`

## Goal
Implement `FromStr` (so `.parse()` works) and `Display` (so `.to_string()` works).

## Why
- `FromStr` is what `str::parse` calls. Its error is `uuid::Error`, passed through unchanged.
- `.map(Self)`: a tuple struct's name is also its constructor function.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/shared/ids.rs`.
