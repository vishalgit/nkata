# Load the data table

**File:** `style/components/_index.scss` (part 3 of 3)  
**Save to:** `~/code/todo/style/components/_index.scss`

## Goal
Add `@use 'data-table';` to the components index.

## Promote yourself — locally and in Kubernetes
```sh
$ docker compose exec db psql -U todo -d todo_dev -c "UPDATE users SET role = 'admin' WHERE email = 'you@example.com'"
$ kubectl -n todo exec statefulset/postgres -- psql -U todo -d todo -c "UPDATE users SET role = 'admin' WHERE email = 'you@example.com'"
```
Or `:DB UPDATE users SET role = 'admin' WHERE email = 'you@example.com'`. Every request re-reads the role through the session join, so a demoted admin loses access on the next click.

| Layer | Responsibility | Tested by |
| --- | --- | --- |
| `domain::identity::authorize` | The rules: who may do what | Three unit tests |
| `features/todos/access.rs` | Load + authorize + hide existence | HTTP tests (404 for others) |
| Each slice's `shell` | Ask for the right `Action` | HTTP tests per endpoint |
| UI | Hide buttons and links people cannot use | Component and E2E tests |

## When it passes
**◀ PR** `feat(admin): view all todos and moderate (TODO-19, TODO-18)` · labels `type:task,type:story,layer:http,epic:E4` · closes both tasks and the stories TODO-19 and TODO-18 (the data table was TODO-18's last task)

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,type:story,layer:http,epic:E4
$ gh pr ready && gh pr checks --watch && gh pr merge --squash --delete-branch
```

**Checkpoint.** All layers green; close *Ch 10 · Authorization*; merge the Release PR.

## Help
- Save with `:w! ~/code/todo/style/components/_index.scss`.
