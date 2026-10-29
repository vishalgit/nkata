# bin/deploy — build, load, roll out, verify

**File:** `bin/deploy` (part 1 of 2)  
**Save to:** `~/code/todo/bin/deploy`

## Goal
Type `bin/deploy`: one command from source to a smoke-tested deployment.

## Why
- The tag defaults to the current commit, so every image says exactly which code it holds.
- `k3d image import` copies the image into the cluster's nodes: no registry needed.
- `sed` swaps the placeholder `todo:dev` for the real tag; the manifests stay generic.
- The smoke test retries for a minute, then fails the script — the pipeline's deliver stage relies on that exit code.

## When it passes
```sh
$ chmod +x bin/deploy
$ bin/deploy
$ kubectl -n todo get pods                       # two todo pods, one postgres
$ kubectl -n todo exec deploy/todo -- sh         # fails: no shell in the image — the point
```
Open `http://localhost:8080`: the same **Todo** page, served by two pods in Kubernetes.

**◀ PR** `feat(k8s): manifests and bin/deploy (TODO-3)` · labels `type:task,type:enabler,layer:infra,epic:E0` · closes the task and TODO-3

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,type:enabler,layer:infra,epic:E0
$ gh pr ready && gh pr merge --squash --delete-branch
```

## Help
- Save with `:w! ~/code/todo/bin/deploy`.
