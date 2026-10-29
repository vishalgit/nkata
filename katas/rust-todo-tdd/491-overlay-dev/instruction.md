# The dev overlay

**File:** `deploy/k8s/overlays/dev/kustomization.yaml`  
**Save to:** `~/code/todo/deploy/k8s/overlays/dev/kustomization.yaml`

## Goal
A namespace, one replica, and a host name.

## Why
- The `namespace:` line rewrites every resource from the base, the Namespace object included.
- The patches are JSON patches: `replace` the replica count, `add` a host to the first ingress rule.

## Help
- Save with `:w! ~/code/todo/deploy/k8s/overlays/dev/kustomization.yaml`.
