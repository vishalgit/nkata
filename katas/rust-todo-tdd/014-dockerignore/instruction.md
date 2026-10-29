# Keep the build context small — .dockerignore

**File:** `.dockerignore` (part 1 of 2)  
**Save to:** `~/code/todo/.dockerignore`

## Goal
Type `.dockerignore`.

## Why
It keeps the database, the build output and Git history out of every image's build context: faster builds, and no data baked into an image.

## Help
- Save with `:w! ~/code/todo/.dockerignore`.
