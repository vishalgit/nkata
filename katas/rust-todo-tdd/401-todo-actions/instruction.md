# Item actions, shared through context

**File:** `crates/app/src/features/todos/actions.rs` (part 1 of 3)  
**Save to:** `~/code/todo/crates/app/src/features/todos/actions.rs`

**▶ Task**
```sh
scripts/backlog task TODO-8 9 ui "Done and Undo buttons (9.6–9.7)" done-undo
```

## Goal
↪ Continues **TODO-8** — the Done and Undo buttons. Every item needs the item actions, and the list must refetch when any finishes: one struct holds them all.

## Why
- `versions()` reads every action's version, so a resource using it as its source refetches after any of them.
- `error()` shows whichever action failed most visibly; the board renders it once, above the list.
- Rename and delete join this struct when their slices arrive.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/actions.rs`.
