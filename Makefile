.PHONY: test bdd-test build clean build-release install lint \
	check-format format coverage

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

format:
	cargo fmt --all
	taplo format

coverage:
	cargo llvm-cov --locked --all-targets --html --output-dir target/coverage/html
	cargo llvm-cov report --locked --lcov --output-path target/coverage/lcov.info
