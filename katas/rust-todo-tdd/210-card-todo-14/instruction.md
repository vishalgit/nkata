# Story card TODO-14 · Create an account

**File:** `backlog/stories/TODO-14.md`  
**Save to:** `~/code/todo/backlog/stories/TODO-14.md`

## Goal
Write the card for **TODO-14 · Create an account** (E3 My account, 8 points).

Authentication answers *who are you?* It is built like todos: pure domain types and workflows first, then thin adapters for Argon2, cookies and Postgres. Your own session handling is about 60 lines — exactly what libraries like `axum-login` hide.

- Passwords are hashed with **Argon2id**; only the PHC hash string is stored.
- Login creates a random 256-bit **session token** in an `HttpOnly; SameSite=Lax` cookie. The database stores only its **SHA-256 hash**, so a leaked `sessions` table cannot be replayed.
- Sessions expire after 7 days; logout deletes the row.

The order: TODO-14's domain types and registration, then TODO-15's `authenticate` (the users store returns its input type), then the users table, then sessions. Each story brings its own table.

## Why
- The card is the issue body: `scripts/backlog story` reads `backlog/stories/TODO-14.md`.
- Each acceptance criterion names the test that will prove it. The story closes when every one exists and passes.
- Commit the card with the story's first task.

## When it passes
Create the issues from this card:

```sh
$ scripts/backlog epic E3 "My account" "Register, log in and out safely"
$ scripts/backlog story TODO-14 E3 story 8 "Create an account"
```

A new epic starts here, so its issue comes first.

## Help
- Save with `:w! ~/code/todo/backlog/stories/TODO-14.md`.
