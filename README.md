# gvm-rs
A Rust library for connecting to the Greenbone Vulnerability Management API

## BDD Tests

The repository includes BDD tests that run against a local gvmd instance through its Unix socket.

Run them from the repository root:

```bash
cargo run -p gvm-rs-bdd-tests
```

The local gvmd stack and Unix socket must be available before running the tests.

## Documentation

- [BDD test environment variables](bdd-tests/docs/envireonment_variables.md)
