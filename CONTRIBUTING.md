## Contributing

We welcome pull requests and issue reports. To keep releases predictable:

1. Fork the repository and create a topic branch (`git checkout -b feat/my-change`).
2. Follow the [Conventional Commits](https://www.conventionalcommits.org/) spec (`type(scope): summary`).
3. For user-facing changes, update documentation and add tests when practical (`make test`).
4. Make sure `make lint` and `make build` complete without errors before opening a pull request.
5. We publish releases with semantic versioning—highlight breaking changes in the PR description so we can bump the major version when necessary.

## Development

### Prerequisites

- Rust toolchain (1.91 or newer). Confirm with:

  ```bash
  rustc -V
  cargo --version
  ```

- Protocol Buffers compiler (`protoc`) on your `PATH`. Typical install commands:

  ```bash
  # macOS
  brew install protobuf

  # Debian / Ubuntu
  sudo apt-get install -y protobuf-compiler
  ```

  You can also point the build to a custom binary by exporting `PROTOC=/path/to/protoc`.

### Build

```bash
make build
```

### Lint

```bash
make lint
```

### Build Container Images

```bash
make image TAG=dev
```

- Override `IMAGE` or `TAG` to match your registry and tag strategy.
- Use `make image-multiarch PUSH=1` to build and push a multi-architecture image via `docker buildx`.

### Command-line Arguments

| Flag | Environment variable | Description | Default |
| --- | --- | --- | --- |
| `--driver-name` | `DRIVER_NAME` | CSI driver identifier registered with Kubernetes. | `lustre.csi.klustrefs.io` |
| `--node-id` | `KUBE_NODE_NAME` | Unique node identifier reported to the control plane. | Required |
| `--endpoint` | `CSI_ENDPOINT` | Unix socket where the gRPC server listens. | `/var/lib/kubelet/plugins/lustre.csi.klustrefs.io/csi.sock` |
| `--log-level` | `LOG_LEVEL` | Log verbosity (`trace`, `debug`, `info`, `warn`, `error`). | `info` |

Deployments typically set these values through the DaemonSet manifest, but you can override them for local runs or custom automation.
