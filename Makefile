.PHONY: check
check:
	cargo clippy
	cargo audit

.PHONY: build
build:
	cargo build --release
