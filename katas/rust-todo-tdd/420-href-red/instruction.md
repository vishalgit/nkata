# Links that remember the whole query — test 1

**File:** `crates/app/src/features/todos/list.rs` (part 16 of 25)  
**Save to:** `~/code/todo/crates/app/src/features/todos/list.rs`

**▶ Task**
```sh
TASK_ONLY=1 scripts/backlog task TODO-10 9 ui "Filter tabs (9.8)"
scripts/backlog task TODO-12 9 ui "list_href and the pager (9.8–9.9)" pager-and-tabs
```

## Goal
↪ Continues **TODO-12 and TODO-10** — `list_href`, the pager and the filter tabs. Every filter tab and pager link must keep the other parameters; one pure function builds them. Add its first test.

## Why
A plain `#[cfg(test)]` module, not `ssr`: `list_href` is pure and runs in both builds.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/list.rs`.
