# Green — reopen

**File:** `crates/domain/src/todos/todo.rs` (part 8 of 10)  
**Save to:** `~/code/todo/crates/domain/src/todos/todo.rs`

## Goal
Add the reverse transition to `CompletedTodo`.

## Feel the compiler work for you
Try these in a test, read each error, then delete them:
```rust
let active = ActiveTodo::new(info());
let done = active.complete(at(10));
active.complete(at(11));   // error[E0382]: use of moved value: `active`
done.complete(at(12));     // error[E0599]: no method named `complete` found for `CompletedTodo`
```
No runtime check and no extra test is needed for these mistakes.

## When it passes
**◀ PR** `feat(domain): one type per todo state (TODO-8)` · labels `type:task,layer:domain,epic:E1`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:domain,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/crates/domain/src/todos/todo.rs`.
