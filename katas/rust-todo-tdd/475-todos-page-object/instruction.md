# TodosPage — the life of a todo

**File:** `end2end/tests/pages/todos-page.ts` (part 1 of 5)  
**Save to:** `~/code/todo/end2end/tests/pages/todos-page.ts`

**▶ Task**
```sh
for s in TODO-8 TODO-9 TODO-13; do TASK_ONLY=1 scripts/backlog task "$s" 11 e2e "Lifecycle journey (11.6)"; done
scripts/backlog task TODO-7 11 e2e "Lifecycle journey (11.6)" lifecycle-journey
```

## Goal
↪ Completes **TODO-7, TODO-8, TODO-9 and TODO-13** — the lifecycle journey is the last task of all four (one PR closes four task issues and four stories). Type the todos page object.

## Why
- Every action waits for its visible result (`toBeVisible`, `toHaveClass`, `toHaveCount(0)`), so the next step never races the previous one.
- `open` is imported now for `visit`, which a later journey adds.
- `data-testid="todo-item"` from Chapter 8 is what `items()` finds.

## Help
- Save with `:w! ~/code/todo/end2end/tests/pages/todos-page.ts`.
