# The first migration

**File:** `migrations/<timestamp>_create_todos.sql`  
**Save to:** `~/code/todo/migrations/<timestamp>_create_todos.sql`

**▶ Task**
```sh
TASK_ONLY=1 scripts/backlog task TODO-4 5 infra "Migrations and offline query data in the pipeline (5.15)"
scripts/backlog task TODO-7 5 persistence "todos table, insert and find (5.1–5.5)" todos-table
```

## Goal
↪ Continues **TODO-7** — the `todos` table, `insert` and `find`. This PR also closes **TODO-4 · Ch 5 · Migrations and offline query data in the pipeline**. Create the migration and type the table.

## Run first
```sh
$ bin/dev sqlx migrate add create_todos
```

## Why
- The `CHECK` constraints repeat the domain rules on purpose: defence in depth against a future script that bypasses the domain.
- `completed_at` is nullable: `NULL` means active. The domain's two states become one nullable column.
- The index matches the list query: one owner's todos, newest first.

## Check
```sh
$ bin/dev sqlx migrate run
```
Check it with your database client on the host: `select * from _sqlx_migrations`.

Every store test in this chapter runs against a real, throwaway Postgres database created for that test: the SQL you test is the SQL you ship.

## Help
- Save with `:w! ~/code/todo/migrations/<timestamp>_create_todos.sql` (the timestamp is the one sqlx gave the file).
