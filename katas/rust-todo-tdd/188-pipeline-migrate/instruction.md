# The pipeline learns about the database

**File:** `.github/workflows/pipeline.yml` (part 4 of 7)  
**Save to:** `~/code/todo/.github/workflows/pipeline.yml`

## Goal
This is TODO-4's Chapter 5 task, and it must ride in this PR: CI must migrate the database before anything compiles `query!`, run the integration layer, and fail when `.sqlx/` is stale.

## Why
- The first `query!` checks SQL at compile time, so this PR could not even compile in CI without a migrated database.
- **Commit the query metadata** so the image builds without a database:
  ```sh
  $ bin/dev cargo sqlx prepare --workspace -- -p server
  $ ls .sqlx
  ```
  `-- -p server` type-checks through the server crate, which compiles `app` with `ssr` only.
- Forgetting to refresh `.sqlx/` after changing a query now fails the PR.

## Help
- Save with `:w! ~/code/todo/.github/workflows/pipeline.yml`.
