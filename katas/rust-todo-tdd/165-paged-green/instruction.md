# Green — Paged<T>

**File:** `crates/domain/src/todos/query.rs` (part 6 of 14)  
**Save to:** `~/code/todo/crates/domain/src/todos/query.rs`

## Goal
A page of results, generic over the item type.

## Why
- Generic: the domain pages `Todo`s, the test pages numbers, the HTTP layer will page views. One type, one set of rules.
- `div_ceil` rounds up: 25 items at 10 per page is 3 pages. `.max(1)` makes an empty list one (empty) page.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/query.rs`.
