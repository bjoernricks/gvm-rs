# gvm-rs

A Rust library for connecting to the Greenbone Vulnerability Management API

## CLI

The `gvm-cli` binary is enabled by the default `cli` feature.

CLI configuration can be provided either by command-line flags or environment variables:

| Flag            | Environment variable | Default                   |
| --------------- | -------------------- | ------------------------- |
| `--socket-path` | `GVM_SOCKET_PATH`    | `/tmp/gvm/gvmd/gvmd.sock` |
| `--username`    | `GVM_USERNAME`       | required                  |
| `--password`    | `GVM_PASSWORD`       | required                  |

The CLI also loads a local `.env` file before parsing arguments, so the same environment variables can be stored there.

Example:

```bash
cargo run -- --socket-path /run/gvmd/gvmd.sock --username admin --password admin
```

Or with environment variables:

```bash
GVM_SOCKET_PATH=/run/gvmd/gvmd.sock GVM_USERNAME=admin GVM_PASSWORD=admin cargo run --
```

Or with a `.env` file:

```env
GVM_SOCKET_PATH=/run/gvmd/gvmd.sock
GVM_USERNAME=admin
GVM_PASSWORD=admin
```

## Async Support

The crate provides an optional Tokio-based async client behind the `async-tokio` feature.

Enable the feature in your dependency:

```toml
[dependencies]
gvm-rs = { version = "0.1.0", features = ["async-tokio"] }
```

Use `GmpAsyncClient`:

```rust
use gvm_rs::{
    async_client::GmpAsyncClient,
    commands::version::{GetVersionRequest, GetVersionResponse},
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut client = GmpAsyncClient::from_unix_socket_path("/tmp/gvm/gvmd/gvmd.sock")
        .await
        .expect("failed to connect");

    let response: GetVersionResponse = client
        .send_command(&GetVersionRequest)
        .await
        .expect("failed to send command");

    println!("{:?}", response);
}
```

Run async tests with:

```bash
cargo test --features async-tokio
```

## BDD Tests

The repository includes BDD tests that run against a local gvmd instance through its Unix socket.

Run them from the repository root:

```bash
cargo run -p gvm-rs-bdd-tests
```

The local gvmd stack and Unix socket must be available before running the tests.

## Documentation

- [BDD test environment variables](bdd-tests/docs/environment_variables.md)
