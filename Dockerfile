# Multi-stage build with Rust 1.90 and Alpine
FROM rust:1.90-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    protobuf-dev \
    protoc

WORKDIR /build

# Copy manifests first for better caching
COPY Cargo.toml Cargo.lock build.rs ./
COPY proto ./proto

# Create dummy main to cache dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-musl && \
    rm -rf src

# Copy real source code
COPY src ./src

# Build for real with static linking
ENV RUSTFLAGS='-C target-feature=+crt-static -C link-arg=-static'
RUN cargo build --release --target x86_64-unknown-linux-musl

# Verify it's static
RUN file /build/target/x86_64-unknown-linux-musl/release/klustrefs-csi-plugin && \
    ldd /build/target/x86_64-unknown-linux-musl/release/klustrefs-csi-plugin || true

# Runtime stage - minimal Alpine
FROM alpine:3.22

# Install only essential runtime tools
RUN apk add --no-cache \
    ca-certificates \
    util-linux \
    kmod

# Copy static binary from builder
COPY --from=builder \
    /build/target/x86_64-unknown-linux-musl/release/klustrefs-csi-plugin \
    /usr/local/bin/klustrefs-csi-plugin

# Set permissions and create socket directory
RUN chmod +x /usr/local/bin/klustrefs-csi-plugin && \
    mkdir -p /csi

# Add non-root user (optional, CSI needs root for mount)
# RUN addgroup -g 1000 csi && adduser -D -u 1000 -G csi csi

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD ["/usr/local/bin/klustrefs-csi-plugin", "--help"]

ENTRYPOINT ["/usr/local/bin/klustrefs-csi-plugin"]
CMD ["--help"]
