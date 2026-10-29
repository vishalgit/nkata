# A trigram index for fast substring search

**File:** `migrations/<timestamp>_todos_title_search.sql`  
**Save to:** `~/code/todo/migrations/<timestamp>_todos_title_search.sql`

**▶ Task**
```sh
scripts/backlog task TODO-11 5 persistence "Trigram index, LIKE escaping, SQL search (5.9–5.12)" sql-search
```

## Goal
↪ Continues **TODO-11 · Search my todos**. `ILIKE '%milk%'` cannot use a B-tree index; `pg_trgm` indexes three-letter fragments.

## Run first
```sh
$ bin/dev sqlx migrate add todos_title_search
```

## Why
- The `todo` role owns the database in Compose and Kubernetes, so it may create the extension.
- With a few thousand rows, `:DB EXPLAIN SELECT * FROM todos WHERE title ILIKE '%milk%'` shows a `Bitmap Index Scan on todos_title_trgm_idx`.

## Check
```sh
$ bin/dev sqlx migrate run
```

## Help
- Save with `:w! ~/code/todo/migrations/<timestamp>_todos_title_search.sql` (the timestamp is the one sqlx gave the file).
