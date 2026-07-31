.PHONY: test bdd-test build clean build-release install lint \
	check-format format install-dev-tools

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
	taplo format --check

format:
	cargo fmt --all
	taplo format

install-dev-tools:
	cargo install taplo-cli --locked
