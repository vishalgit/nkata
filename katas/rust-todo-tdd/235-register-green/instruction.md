# Green — a fail-fast first version

**File:** `crates/domain/src/identity/register.rs` (part 2 of 5)  
**Save to:** `~/code/todo/crates/domain/src/identity/register.rs`

## Goal
The hash type, the command, the event, the errors, and a workflow generic over its hashing function.

## Why
- `H: FnOnce(&Password) -> Result<PasswordHash, HashingFailed>` is dependency injection with a function type: production passes Argon2, tests pass a fake.
- `PasswordHash` hides itself in `Debug` too — a hash is not a secret, but it is still nobody's business in a log.
- `HashingFailed` carries no details on purpose: crypto errors are not for users.
- Every new user is `Role::User`. Admins are made by an operator, never by registering.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/identity/register.rs`.
