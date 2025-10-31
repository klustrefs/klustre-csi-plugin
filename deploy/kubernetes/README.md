# Kubernetes Manifests

This directory contains Kustomize assets for deploying the Klustre CSI driver.

## Layout
- `base/` – baseline resources (namespace, RBAC, CSIDriver, DaemonSet, sample StorageClass/PV/PVC) parameterized via ConfigMap-driven replacements.
- `overlays/` – environment-specific customizations; see `overlays/example` for reference values.

## Usage
```bash
# Render the base manifests
kustomize build deploy/kubernetes/base

# Render the example overlay
kustomize build deploy/kubernetes/overlays/example

# Apply to a cluster
kustomize build deploy/kubernetes/base | kubectl apply -f -
```

Adjust the config-map literals in an overlay to tune images, socket paths, storage class attributes, and other deployment details. The namespace (`klustre-system`) and CSI driver name (`lustre.csi.klustrefs.io`) are fixed in the base manifests.
