IMAGE ?= ghcr.io/klustrefs/klustre-csi-plugin
TAG ?= dev
PLATFORM ?= linux/amd64
PLATFORMS ?= linux/amd64,linux/arm64
BUILD_DIR ?= build

.PHONY: deps build fmt fmt-fix lint test image image-multiarch clean

deps:
	cargo fetch
	rustup component add clippy rustfmt >/dev/null
	rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl >/dev/null

build:
	cargo build --release

fmt:
	cargo fmt -- --check

fmt-fix:
	cargo fmt

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --all

image:
	docker buildx build \
		--platform=$(PLATFORM) \
		-t $(IMAGE):$(TAG) \
		-f Dockerfile \
		--load \
		.

image-multiarch:
	@if [ "$(PUSH)" = "1" ]; then \
	  docker buildx build \
		--platform=$(PLATFORMS) \
		-t $(IMAGE):$(TAG) \
		-f Dockerfile \
		--push \
		. ; \
	else \
	  mkdir -p $(BUILD_DIR)/$(TAG); \
	  docker buildx build \
		--platform=$(PLATFORMS) \
		-t $(IMAGE):$(TAG) \
		-f Dockerfile \
		--output=type=oci,dest=$(BUILD_DIR)/$(TAG)/klustre-csi-plugin.tar \
		. ; \
	  echo "Multi-arch image archived at $(BUILD_DIR)/$(TAG)/klustre-csi-plugin.tar"; \
	  echo "Load with 'docker load < $(BUILD_DIR)/$(TAG)/klustre-csi-plugin.tar' or push with 'docker buildx build --push'."; \
	fi


clean:
	cargo clean
	rm -rf $(BUILD_DIR)
