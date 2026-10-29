# _todos.scss

**File:** `style/pages/_todos.scss`  
**Save to:** `~/code/todo/style/pages/_todos.scss`

## Goal
The board's own layout.

## Check
cargo-leptos recompiles `style/main.scss` on every save and the browser reloads. Check both colour schemes and tab through the forms: focus rings must show.

Then the chapter checkpoint:
```sh
$ bin/dev cargo t-int && bin/dev cargo t-http && bin/e2e
$ bin/deploy
```
Open `http://localhost:8080`, register, add a todo — in Kubernetes.

## When it passes
**◀ PR** `feat(ui): 7-1 SCSS with light and dark themes (TODO-18)` · labels `type:task,layer:ui,epic:E1`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:ui,epic:E1
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

**Checkpoint.** Close *Ch 08 · UI and 7-1 SCSS*; merge the Release PR.

## Help
- Save with `:w! ~/code/todo/style/pages/_todos.scss`.
