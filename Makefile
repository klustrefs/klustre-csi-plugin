IMAGE ?= ghcr.io/klustrefs/klustre-csi-plugin
TAG ?= dev
PLATFORM ?= linux/amd64
PLATFORMS ?= linux/amd64,linux/arm64
BUILD_DIR ?= build
VERSION ?=

.PHONY: deps build fmt fmt-fix lint test image image-multiarch clean check-manifest-version tag tag-push tag-delete tag-repush release

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

check-manifest-version:
	@if [ -z "$(VERSION)" ]; then echo "ERROR: VERSION is required (e.g., VERSION=v0.1.0)" >&2; exit 1; fi
	@IMAGE_TAG=$$(grep -A2 'name: ghcr.io/klustrefs/klustre-csi-plugin' manifests/kustomization.yaml | grep 'newTag:' | awk '{print $$2}'); \
	if [ -z "$$IMAGE_TAG" ]; then \
	  echo "ERROR: Could not determine image tag from manifests/kustomization.yaml" >&2; exit 1; \
	fi; \
	if [ "$$IMAGE_TAG" != "$(VERSION)" ]; then \
	  echo "ERROR: Image tag in manifests/kustomization.yaml ($$IMAGE_TAG) does not match VERSION ($(VERSION))." >&2; \
	  echo "Update manifests/kustomization.yaml images.newTag before tagging a release." >&2; \
	  exit 1; \
	fi

tag:
	@if [ -z "$(VERSION)" ]; then echo "ERROR: VERSION is required (e.g., VERSION=v0.1.0)" >&2; exit 1; fi
	@case "$(VERSION)" in v*) ;; *) echo "ERROR: VERSION must be v-prefixed (e.g., v0.1.0)" >&2; exit 1;; esac
	@$(MAKE) check-manifest-version VERSION=$(VERSION)
	@if git status --porcelain | grep -q '.'; then echo "ERROR: Working tree is not clean." >&2; exit 1; fi
	@if git rev-parse -q --verify "refs/tags/$(VERSION)" >/dev/null; then \
	  echo "ERROR: Tag $(VERSION) already exists." >&2; exit 1; \
	fi
	@git tag -a $(VERSION) -m "Release $(VERSION)"
	@echo "Created tag $(VERSION)"

tag-push:
	@if [ -z "$(VERSION)" ]; then echo "ERROR: VERSION is required (e.g., VERSION=v0.1.0)" >&2; exit 1; fi
	@$(MAKE) check-manifest-version VERSION=$(VERSION)
	@if ! git rev-parse -q --verify "refs/tags/$(VERSION)" >/dev/null; then \
	  echo "ERROR: Tag $(VERSION) not found. Create it first: make tag VERSION=$(VERSION)" >&2; exit 1; \
	fi
	@git push origin $(VERSION)

tag-delete:
	@if [ -z "$(VERSION)" ]; then echo "ERROR: VERSION is required (e.g., VERSION=v0.1.0)" >&2; exit 1; fi
	@echo "Deleting local tag $(VERSION)"
	@-git tag -d $(VERSION)
	@echo "Deleting remote tag $(VERSION)"
	@-git push --delete origin $(VERSION) || git push origin :refs/tags/$(VERSION)

tag-repush: tag-delete tag
	@$(MAKE) tag-push

release: tag tag-push
