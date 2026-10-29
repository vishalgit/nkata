# The test overlay

**File:** `deploy/k8s/overlays/test/kustomization.yaml`  
**Save to:** `~/code/todo/deploy/k8s/overlays/test/kustomization.yaml`

## Goal
Test is the dev file with `dev` replaced by `test`.

## Help
- `:r !sed 's/dev/test/g' deploy/k8s/overlays/dev/kustomization.yaml` fills the buffer; delete the empty first line.
- Save with `:w! ~/code/todo/deploy/k8s/overlays/test/kustomization.yaml`.
