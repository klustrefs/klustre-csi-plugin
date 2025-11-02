<p align="center">
  <img src="static/klustre_logo_black.svg" alt="KlustreFS logo" width="320">
</p>

# Klustre CSI Plugin

[![CI](https://github.com/klustrefs/klustre-csi-plugin/actions/workflows/build-and-push.yaml/badge.svg)](https://github.com/klustrefs/klustre-csi-plugin/actions/workflows/build-and-push.yaml)
[![Rust Code Quality](https://img.shields.io/badge/Rust%20Code%20Quality-Clippy-success?logo=rust&logoColor=white)](https://doc.rust-lang.org/stable/clippy/)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

A [Container Storage Interface (CSI)](https://kubernetes-csi.github.io/docs/) plugin for Lustre parallel file systems, allowing Kubernetes workloads to use high-performance Lustre storage backends.

## Overview

The KlustreFS CSI plugin connects Kubernetes workloads to Lustre shares by using the
Lustre client available on worker nodes to mount and unmount whenever a PersistentVolume and
PersistentVolumeClaim are created or deleted.

### Current Capabilities

- Mounts and unmounts Lustre shares on Kubernetes worker nodes using the Lustre client.
- Supports Lustre's `ReadWriteMany` semantics for workloads that share mounts across pods.

### Limitations

- Dynamic provisioning workflows (`CreateVolume`, `DeleteVolume`, `ControllerPublish` / `Unpublish`).
- Snapshots, expansion, and metrics (`CreateSnapshot`, `NodeExpandVolume`, `NodeGetVolumeStats`, etc.).

## Prerequisites

### Kubernetes Cluster

- Kubernetes 1.20+ with the CSI spec v1.5 or newer.
- The API server and kubelets must allow privileged pods.

### Worker Nodes

- Lustre client packages installed (`mount.lustre`, kernel modules, etc.).
- Network connectivity to the Lustre shares.

## Quick Start

Use the Kubernetes manifests provided in `manifests/` or Helm chart from the `klustrefs/helm-charts` repository if you prefer Helm.

```bash
kubectl apply -f manifests/
```

Validate the rollout:

```bash
kubectl get pods -n klustre-system
```

### Mount an Existing Lustre Share

Define a PersistentVolume that points at your Lustre export and bind it with a PersistentVolumeClaim.
When a pod uses that PVC, the driver reads `volumeAttributes.source` (for example
`10.0.0.1@tcp0:/lustre-fs`) and mounts the share inside the container.

```yaml
apiVersion: v1
kind: PersistentVolume
metadata:
  name: lustre-static-pv
spec:
  capacity:
    storage: 10Gi
  accessModes:
    - ReadWriteMany
  persistentVolumeReclaimPolicy: Retain
  csi:
    driver: lustre.csi.klustrefs.io
    volumeHandle: lustre-static-pv
    volumeAttributes:
      source: 10.0.0.1@tcp0:/lustre-fs
      mountOptions: flock,user_xattr
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: lustre-static-pvc
spec:
  accessModes:
    - ReadWriteMany
  resources:
    requests:
      storage: 10Gi
  volumeName: lustre-static-pv
  storageClassName: "klustre-csi-static"
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: lustre-demo
spec:
  replicas: 1
  selector:
    matchLabels:
      app: lustre-demo
  template:
    metadata:
      labels:
        app: lustre-demo
    spec:
      containers:
        - name: app
          image: busybox
          command: ["sleep", "infinity"]
          volumeMounts:
            - name: lustre-share
              mountPath: /mnt/lustre
      volumes:
        - name: lustre-share
          persistentVolumeClaim:
            claimName: lustre-static-pvc
```

## Development & Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for build/lint instructions, container image workflows, command-line argument reference, and contribution guidelines.

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
