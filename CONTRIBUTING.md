## Contributing

We welcome pull requests and issue reports. To keep releases predictable:

1. Fork the repository and create a topic branch (`git checkout -b feat/my-change`).
2. Follow the [Conventional Commits](https://www.conventionalcommits.org/) spec (`type(scope): summary`).
3. Sign off every commit (`git commit --amend --no-edit --signoff` if you need to add it retroactively).
4. For user-facing changes, update documentation and add tests when practical (`make test`).
5. Ensure the CI pipeline will pass locally: run `make fmt`, `make lint`, and `make build` before opening a pull request.
6. We publish releases with semantic versioning—highlight breaking changes in the PR description so we can bump the major version when necessary.

### Tagging & Releases

- Only tags prefixed with `v` (for example `v0.1.0`) are accepted by the automation.
- Use the Makefile helpers to keep releases consistent:
  - `make tag VERSION=vX.Y.Z` — create an annotated SemVer tag on the current HEAD.
  - `make tag-push VERSION=vX.Y.Z` — push an existing tag to the `origin` remote.
  - `make release VERSION=vX.Y.Z` — create and push the tag in one step (this triggers the release workflow and container image publish).
- `make tag-delete VERSION=vX.Y.Z` and `make tag-repush VERSION=vX.Y.Z` are available if you need to redo a tag (for example, after fixing a CI failure).

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
