.PHONY: all
all:help

.PHONY: build # cargo build
build:
	RUSTFLAGS="-C link-arg=-s" cargo build --release

.PHONY: fmt # cargo fmt --all
fmt:
	cargo fmt --all

.PHONY: check # cargo clippy --all
check:
	cargo clippy --all

.PHONY: run-dev # run dev node
run-dev:
	./target/release/template-node --tmp --dev --rpc-port 9933

.PHONY: test # cargo test
test:
	cargo test

.PHONY: benchmark # runtime benchmarks
benchmark:
	cargo build --release --features runtime-benchmarks

.PHONY: help # Generate list of targets with descriptions
help:
	@grep '^.PHONY: .* #' Makefile | sed 's/\.PHONY: \(.*\) # \(.*\)/\1	\2/' | expand -t20
