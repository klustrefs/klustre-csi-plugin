.PHONY: build lint

build:
	cargo build --release

lint:
	cargo clippy --all-targets --all-features -- -D warnings
.PHONY: build

build:
	cargo build --release
