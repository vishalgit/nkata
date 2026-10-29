# The sessions table

**File:** `migrations/<timestamp>_create_sessions.sql`  
**Save to:** `~/code/todo/migrations/<timestamp>_create_sessions.sql`

**▶ Task**
```sh
scripts/backlog task TODO-15 6 persistence "Sessions, tokens, cookies, session store (6.12, 6.15–6.16, 6.19–6.20)" sessions
```

## Goal
↪ Continues **TODO-15** — sessions, tokens, cookies and the session store. Create the table.

## Run first
```sh
$ bin/dev sqlx migrate add create_sessions
```

## Why
- The primary key is the token's **hash**: the cookie value itself is never stored.
- `ON DELETE CASCADE`: deleting a user ends every session they had.

## Check
```sh
$ bin/dev sqlx migrate run
```

## Help
- Save with `:w! ~/code/todo/migrations/<timestamp>_create_sessions.sql` (the timestamp is the one sqlx gave the file).
