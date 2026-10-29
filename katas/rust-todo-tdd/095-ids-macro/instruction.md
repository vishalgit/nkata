# Refactor — one macro for every id

**File:** `crates/domain/src/shared/ids.rs` (part 6 of 6)  
**Save to:** `~/code/todo/crates/domain/src/shared/ids.rs`

## Goal
Two identical blocks invite drift: replace everything above the tests with an `id_type!` macro and use it twice.

## Why
- `$(#[$meta:meta])*` passes the doc comment (and any other attribute) through to the generated struct.
- Tests 1 and 2 stay green: the refactor changed how the code is written, not what it does.

## Check
Green. Then add this line inside the test module, read the compile error, and delete it:
```rust
    fn _ids_do_not_mix(u: UserId) -> TodoId { u } // error[E0308]: mismatched types
```

## Help
- Save with `:w! ~/code/todo/crates/domain/src/shared/ids.rs`.
