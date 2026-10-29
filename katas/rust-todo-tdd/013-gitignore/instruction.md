# The data directory and .gitignore

**File:** `.gitignore` (part 1 of 2)  
**Save to:** `~/code/todo/.gitignore`

**▶ Task**
```sh
scripts/backlog task TODO-1 1 infra "Project, .env, Dockerfile.dev, compose.yaml (1.1–1.5)" dev-containers
```

## Goal
Start the first task, create `.pgdata/` yourself, and type `.gitignore`.

## Run first
```sh
$ mkdir -p .pgdata
```

## Why
- Creating `.pgdata/` yourself, before Docker does, makes it belong to uid 1000. A directory Docker creates for you belongs to root.
- `target/` is Cargo's build output; `.pgdata/` is the database.

## Help
- Save with `:w! ~/code/todo/.gitignore`.
