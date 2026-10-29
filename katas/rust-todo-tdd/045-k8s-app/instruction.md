# Manifests — the app

**File:** `deploy/k8s/app.yaml` (part 1 of 2)  
**Save to:** `~/code/todo/deploy/k8s/app.yaml`

## Goal
Type the app's Deployment, Service and Ingress.

## Why
- The readiness probe is the health check you test-drove: a pod only receives traffic once it can reach Postgres.
- `readOnlyRootFilesystem`, `drop: ["ALL"]` and no privilege escalation: the `scratch` image needs none of them.
- `imagePullPolicy: IfNotPresent` uses the image `bin/deploy` imports into k3d instead of pulling from a registry.

## Help
- Save with `:w! ~/code/todo/deploy/k8s/app.yaml`.
