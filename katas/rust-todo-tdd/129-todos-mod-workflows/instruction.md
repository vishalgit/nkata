# The workflows module

**File:** `crates/domain/src/todos/mod.rs` (part 5 of 10)  
**Save to:** `~/code/todo/crates/domain/src/todos/mod.rs`

**▶ Task**
```sh
scripts/backlog task TODO-7 4 domain "create_todo workflow (4.3–4.4)" create-workflow
```

## Goal
↪ Continues **TODO-7 · Add a todo** — the `create_todo` workflow. Declare `workflows` and re-export everything in it.

## Run first
```sh
$ mkdir -p crates/domain/src/todos/workflows
```

## The impure/pure/impure sandwich
A workflow is one use case as a pure function: an unvalidated **command** plus dependencies as plain values in, an **event** or a typed **error** out.

1. **Impure shell, before:** read the request, load from Postgres, read the clock, generate ids.
2. **Pure core:** call the workflow with those values. It decides.
3. **Impure shell, after:** persist the event, shape the response.

The core never awaits, touches the database or reads the clock, so its tests need no mocks and no async runtime.

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/mod.rs`.
