# The base Postgres, without its Secret

**File:** `deploy/k8s/base/postgres.yaml`  
**Save to:** `~/code/todo/deploy/k8s/base/postgres.yaml`

## Goal
Delete the Secret and its `---` from the top of `base/postgres.yaml`.

## Help
- `:1,/^---$/d` deletes from the first line through the first `---`. Or from the shell: `sed -i '1,/^---$/d' deploy/k8s/base/postgres.yaml`.
- `head -2 deploy/k8s/base/postgres.yaml` now shows `apiVersion: v1` / `kind: Service`.
- Save with `:w! ~/code/todo/deploy/k8s/base/postgres.yaml`.
