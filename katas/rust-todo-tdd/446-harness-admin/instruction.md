# The harness can promote a user

**File:** `crates/server/tests/common/mod.rs` (part 7 of 7)  
**Save to:** `~/code/todo/crates/server/tests/common/mod.rs`

**▶ Task**
```sh
TASK_ONLY=1 scripts/backlog task TODO-16 10 http "Route access through the policy (10.5)"
scripts/backlog task TODO-19 10 http "Policy-based access (10.5)" policy-access
```

## Goal
↪ Continues **TODO-19 and TODO-16** — route access through the policy (one PR closes **TODO-19 · Ch 10 · Policy-based access** and **TODO-16 · Ch 10 · Route access through the policy**). There is no UI for promoting users — role changes are an operator task — so tests promote in SQL.

## Help
- Save with `:w! ~/code/todo/crates/server/tests/common/mod.rs`.
