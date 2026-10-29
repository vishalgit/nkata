# Green — two queries, one WHERE

**File:** `crates/app/src/features/todos/store.rs` (part 14 of 21)  
**Save to:** `~/code/todo/crates/app/src/features/todos/store.rs`

## Goal
One query counts, one fetches; both share the same `WHERE`.

## Why
- A `NULL` parameter switches its condition off: `$2::text IS NULL OR title ILIKE $2`. One SQL text serves "no search" too.
- `AS "count!"` tells sqlx the count is never `NULL`, so it arrives as `i64`, not `Option<i64>`.
- `.collect::<Result<Vec<_>, _>>()?` stops at the first corrupt row.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/store.rs`.
