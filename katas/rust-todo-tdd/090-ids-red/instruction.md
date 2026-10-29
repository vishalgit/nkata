# Typed ids — test 1: an id wraps a UUID

**File:** `crates/domain/src/shared/ids.rs` (part 1 of 6)  
**Save to:** `~/code/todo/crates/domain/src/shared/ids.rs`

## Goal
Write the test only: a `TodoId` built from a UUID gives the same UUID back.

## Check
`bin/dev cargo t-unit` — **red**: `unresolved import uuid`.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/shared/ids.rs`.
