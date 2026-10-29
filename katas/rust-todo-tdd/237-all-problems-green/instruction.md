# Green — applicative validation

**File:** `crates/domain/src/identity/register.rs` (part 4 of 5)  
**Save to:** `~/code/todo/crates/domain/src/identity/register.rs`

## Goal
Move validation into its own function that runs every check and collects every problem.

## Why
- **Applicative** validation, rather than the fail-fast `?` railway: both parses run, then the results are combined.
- The second arm runs only when at least one failed. `.err()` turns each `Result` into an `Option` of its error; chaining two `Option`s and collecting gives a list of zero, one or two problems.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/register.rs`.
