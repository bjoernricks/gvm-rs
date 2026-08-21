# gvm-rs

A Rust library for connecting to the Greenbone Vulnerability Management API

## CLI

The `gvm-cli` binary is enabled by the default `cli` feature.

The CLI loads a local `.env` file before parsing arguments. Configuration can be provided by command-line flags or environment variables.

Common options:

| Flag             | Environment variable     | Default    |
| ---------------- | ------------------------ | ---------- |
| `--gmp-username` | `GVM_GMP_USERNAME`       | required   |
| `--gmp-password` | `GVM_GMP_PASSWORD`       | required   |
| `--timeout`      | `GVM_CONNECTION_TIMEOUT` | no timeout |

The timeout is specified in seconds and applies to socket I/O operations.

### Unix socket

| Flag            | Environment variable | Default                   |
| --------------- | -------------------- | ------------------------- |
| `--socket-path` | `GVM_SOCKET_PATH`    | `/tmp/gvm/gvmd/gvmd.sock` |

Example:

```bash
cargo run -- \
    --gmp-username admin \
    --gmp-password admin \
    socket \
    --socket-path /run/gvmd/gvmd.sock
```

### SSH

| Flag                     | Environment variable       | Default     |
| ------------------------ | -------------------------- | ----------- |
| `--ssh-hostname`         | `GVM_SSH_HOSTNAME`         | `localhost` |
| `--ssh-port`             | `GVM_SSH_PORT`             | `22`        |
| `--ssh-username`         | `GVM_SSH_USERNAME`         | `gmp`       |
| `--ssh-password`         | `GVM_SSH_PASSWORD`         | optional    |
| `--ssh-auto-accept-host` | `GVM_SSH_AUTO_ACCEPT_HOST` | `false`     |

Set `--ssh-auto-accept-host` to accept and save unknown SSH host keys in `~/.ssh/known_hosts`.

Example:

```bash
GVM_GMP_USERNAME=admin \
GVM_GMP_PASSWORD=admin \
GVM_SSH_HOSTNAME=localhost \
GVM_SSH_USERNAME=gmp \
cargo run -- ssh
```

The same values can be stored in a `.env` file:

```env
GVM_GMP_USERNAME=admin
GVM_GMP_PASSWORD=admin
GVM_CONNECTION_TIMEOUT=30
GVM_SOCKET_PATH=/run/gvmd/gvmd.sock
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
