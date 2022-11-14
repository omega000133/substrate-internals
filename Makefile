.PHONY: all
all:help

.PHONY: build # cargo build
build:
	RUSTFLAGS="-C link-arg=-s" cargo +nightly build --release

.PHONY: fmt # cargo fmt --all
fmt:
	cargo +nightly fmt --all

.PHONY: check # cargo check
check:
	# cargo +nightly deny check
	# cargo +nightly outdated --exit-code 1
	# cargo +nightly udeps
	# cargo +nightly audit
	# cargo +nightly pants

.PHONY: lint # cargo lint
lint:
	cargo +nightly clippy --all -- -D warnings

.PHONY: run-dev # run dev node
run-dev:
	./target/release/template-node --tmp --dev --rpc-port 9933

.PHONY: test # cargo test
test:
	cargo +nightly test

.PHONY: benchmark # runtime benchmarks
benchmark:
	cargo +nightly build --release --features runtime-benchmarks

.PHONY: help # Generate list of targets with descriptions
help:
	@grep '^.PHONY: .* #' Makefile | sed 's/\.PHONY: \(.*\) # \(.*\)/\1	\2/' | expand -t20
