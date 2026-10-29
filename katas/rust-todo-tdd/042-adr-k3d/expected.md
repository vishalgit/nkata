# 1. k3d for local Kubernetes

Date: 2026-10-20

## Status

Accepted

## Context

The app runs on one Arch machine that already runs Docker for the dev containers. It needs a
Kubernetes cluster for deploys, the pipeline's throwaway cluster, and later dev, test and prod
environments. Options: minikube (a VM, 1–2 GB idle), kind (kubeadm Kubernetes in Docker,
~500–700 MB), k3d (k3s in Docker, ~300–500 MB).

## Decision

Use k3d. It reuses the Docker already running, starts in seconds, ships Traefik ingress and local
storage, and imports locally built images with `k3d image import`.

## Consequences

- The cluster is k3s, not upstream kubeadm Kubernetes; conformance-level differences are accepted.
- Ingress is Traefik: `--port 8080:80@loadbalancer` maps `localhost:8080` to it.
- CI creates the same kind of cluster with the k3d install script.
