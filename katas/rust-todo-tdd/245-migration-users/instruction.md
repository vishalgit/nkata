# The users table, and a real owner for every todo

**File:** `migrations/<timestamp>_create_users.sql`  
**Save to:** `~/code/todo/migrations/<timestamp>_create_users.sql`

**▶ Task**
```sh
scripts/backlog task TODO-14 6 persistence "Users table, Argon2 adapter, insert_user (6.12–6.14, 6.17–6.18)" users-table
```

## Goal
↪ Continues **TODO-14** — the users table, Argon2 and `insert_user`. Create the migration: users, then a foreign key from every todo to its owner.

## Run first
```sh
$ bin/dev sqlx migrate add create_users
```

## Check
```sh
$ bin/dev sqlx migrate run
$ bin/dev cargo t-int
```
**Red — good news.** The Chapter 5 store tests fail with `violates foreign key constraint "todos_owner_fk"`: the schema changed and the tests caught it. Fix the tests, not the schema (next kata).

## Help
- Save with `:w! ~/code/todo/migrations/<timestamp>_create_users.sql` (the timestamp is the one sqlx gave the file).
