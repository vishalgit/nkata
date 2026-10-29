# A local cluster, and the first ADR

**File:** `docs/adr/0001-k3d-for-local-kubernetes.md`  
**Save to:** `~/code/todo/docs/adr/0001-k3d-for-local-kubernetes.md`

**▶ Task**
```sh
scripts/backlog task TODO-3 2 infra "k3d cluster (2.14)" k3d
```

## Goal
Create a k3d cluster, then record the decision as the first ADR: context, decision, consequences.

## Run first
```sh
$ sudo pacman -S kubectl k3d        # if k3d is missing from your mirror: yay -S k3d-bin
$ k3d cluster create todo --agents 1 --port "8080:80@loadbalancer" --wait
$ kubectl get nodes                 # one server, one agent: Ready
```

## Why
| Tool | Runs nodes as | Idle memory (approx.) | Good at |
| --- | --- | --- | --- |
| minikube | a VM (or a container) | 1–2 GB with a VM | addons, dashboard; the heaviest |
| kind | Docker containers running kubeadm Kubernetes | ~500–700 MB | CI, conformance |
| **k3d** | Docker containers running **k3s** | ~300–500 MB | lightest; Traefik ingress, local storage, `image import` built in |

- `--port 8080:80@loadbalancer` maps `localhost:8080` to the cluster's ingress.
- `k3d cluster stop todo` / `start todo` frees and reclaims memory; `k3d cluster delete todo` starts over.
- The decision outlives this PR, so it goes in an ADR, not a commit message.

## When it passes
Delete `docs/adr/.gitkeep` (`git rm docs/adr/.gitkeep`).

**◀ PR** `docs(adr): k3d for local Kubernetes (TODO-3)` · labels `type:task,layer:infra,epic:E0`

```sh
$ git push -u origin HEAD
$ gh pr create --draft --fill --label type:task,layer:infra,epic:E0
$ gh pr ready && gh pr merge --squash --delete-branch
```

## Help
- The wording is yours to change; keep the four headings.
- Save with `:w! ~/code/todo/docs/adr/0001-k3d-for-local-kubernetes.md`.
