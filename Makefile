.PHONY: test bdd-test build clean build-release install lint \
	check-format format coverage install-taplo-cli install-llvm-cov \
	install-dev-tools

INSTALL_PREFIX ?= /usr/local

test:
	cargo test

bdd-test:
	cargo run -p gvm-rs-bdd-tests

build:
	cargo build --verbose

build-release:
	cargo build --release --verbose

clean:
	cargo clean

install:
	cargo install --path . --root $(DESTDIR)$(INSTALL_PREFIX)

lint:
	cargo clippy --all-targets -- -D warnings

check-format:
	cargo fmt --all -- --check

format: install-taplo-cli
	cargo fmt --all
	taplo format

install-taplo-cli:
	cargo install --locked taplo-cli

install-llvm-cov:
	cargo install --locked cargo-llvm-cov

install-dev-tools: install-taplo-cli install-llvm-cov

coverage: install-llvm-cov
	cargo llvm-cov --locked --all-targets --html --output-dir target/coverage/html
	cargo llvm-cov report --locked --lcov --output-path target/coverage/lcov.info
