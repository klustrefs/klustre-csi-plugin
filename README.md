# KlustreFS CSI Driver

A [Container Storage Interface (CSI)](https://kubernetes-csi.github.io/docs/) driver for Lustre parallel filesystems, enabling Kubernetes workloads to use high-performance Lustre storage.

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

## Features

- ✅ **Static Provisioning** - Mount existing Lustre filesystems
- ✅ **ReadWriteMany (RWX)** - Multiple pods can access the same volume simultaneously
- ✅ **High Performance** - Leverage Lustre's parallel I/O capabilities
- ✅ **Broad Compatibility** - Uses nsenter to work with various container runtimes and base images
- ✅ **Kubernetes Native** - Standard CSI interface, works with any Kubernetes cluster

## How It Works

The driver uses `nsenter` to execute mount operations in the host's mount namespace, allowing it to work with:

- **Alpine-based containers** - No glibc compatibility issues
- **Minimal images** - Doesn't require Lustre utilities inside the container
- **Any base image** - Works regardless of container distro
- **Standard Kubernetes** - No special node configuration needed

## Architecture

The CSI driver consists of two main components:

- **Node Plugin** (DaemonSet) - Runs on every Kubernetes worker node, handles mount operations
- **Controller Plugin** (Deployment) - Manages volume lifecycle (create/delete operations)

## Prerequisites
### Lustre Requirements

- Lustre filesystem deployed and accessible via LNET
- Lustre client software installed on all Kubernetes worker nodes
- Network connectivity between Kubernetes nodes and Lustre MGS/OSTs

### Kubernetes Requirements

- Kubernetes 1.20+
- CSI spec 1.5.0+
- `--allow-privileged` flag enabled on API server and kubelets

## Installation

Deploy the CSI driver:
```bash
kubectl apply -k deploy/kubernetes/base/
```

Verify installation:
```bash
kubectl get pods -n klustre-system -l app=klustre-csi-node
```

## Building (Developers)

### Prerequisites
- [Rust](https://rustup.rs/) toolchain (stable)
- Protocol Buffers compiler (`protoc`)
  - macOS: `brew install protobuf`
  - Debian/Ubuntu: `sudo apt-get install -y protobuf-compiler`
  - Or point the build to a custom binary via `PROTOC=/path/to/protoc`

### Build the driver
```bash
make build
```

The `Makefile` wraps `cargo build --release`; ensure `protoc` is on your `PATH` (or set `PROTOC=/custom/bin/protoc make build`) before invoking it.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
