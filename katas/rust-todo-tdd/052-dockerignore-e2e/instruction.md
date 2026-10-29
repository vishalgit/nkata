# …and out of image builds

**File:** `.dockerignore` (part 2 of 2)  
**Save to:** `~/code/todo/.dockerignore`

## Goal
Append `end2end` to `.dockerignore`, then create the tests folder.

## Run first
```sh
$ mkdir -p end2end/tests
```

## Why
The production image never needs the browser tests, and `node_modules` is large.

## Help
- Save with `:w! ~/code/todo/.dockerignore`.
