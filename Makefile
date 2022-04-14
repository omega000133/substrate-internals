.PHONY: all
all:test build
	@echo "all done"

.PHONY: build
build:
	cargo build --release

.PHONY: run-dev
run-dev:
	./target/release/template-node --tmp --dev --rpc-port 9933

.PHONY: test
test:
	cargo test

.PHONY: benchmark
benchmark:
	cargo build --release --features runtime-benchmarks
