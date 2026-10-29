# Fix the tests, not the schema

**File:** `crates/app/src/features/todos/store.rs` (part 21 of 21)  
**Save to:** `~/code/todo/crates/app/src/features/todos/store.rs`

## Goal
Replace the `user` helper with one that inserts a real user, then update every call site.

Update every call site with one Neovim substitution:
```vim
:%s/user(\(\d\))/user(\&pool, \1).await/g
```

## Why
- `ON CONFLICT (id) DO NOTHING`: tests call `user(&pool, 1)` several times; the second insert is a no-op.
- Before this fix, `the_database_refuses_an_overlong_title` would have passed for the *wrong* reason — the foreign key, not the `CHECK`. A test that passes for the wrong reason is a bug in the test.

## Check
`bin/dev cargo t-int` — green again.

## Help
- Save with `:w! ~/code/todo/crates/app/src/features/todos/store.rs`.
