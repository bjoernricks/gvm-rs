// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use super::check_server_key_against_file;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const TEST_KEY: &str =
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIKQ9zpZ/tmqeuKyEnhrR0SNkZ6pxdMMNFqQdrI8UvuSq";

fn test_path(name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "gvm-rs-ssh-{name}-{}-{timestamp}",
        std::process::id()
    ))
}

fn test_key() -> russh::keys::PublicKey {
    russh::keys::PublicKey::from_openssh(TEST_KEY).expect("failed to parse test key")
}

#[test]
fn missing_known_hosts_file_rejects_unknown_host() {
    let path = test_path("missing");

    let result = check_server_key_against_file(
        "scanner.example",
        22,
        &test_key(),
        Some(path.clone()),
        false,
    )
    .expect("unknown host should be rejected by the handler");

    assert!(!result);
    assert!(!path.exists());
}

#[test]
fn auto_accept_writes_unknown_host_to_known_hosts_file() {
    let path = test_path("accepted");

    let result = check_server_key_against_file(
        "scanner.example",
        2222,
        &test_key(),
        Some(path.clone()),
        true,
    )
    .expect("failed to accept unknown host");

    assert!(result);
    let content = std::fs::read_to_string(&path).expect("failed to read known_hosts file");
    assert!(content.starts_with("[scanner.example]:2222 ssh-ed25519 "));
    std::fs::remove_file(path).expect("failed to remove known_hosts file");
}

#[test]
fn matching_known_host_is_accepted() {
    let path = test_path("matching");
    std::fs::write(&path, format!("scanner.example {TEST_KEY}\n"))
        .expect("failed to write known_hosts file");

    let result = check_server_key_against_file(
        "scanner.example",
        22,
        &test_key(),
        Some(path.clone()),
        false,
    )
    .expect("failed to check known host");

    assert!(result);
    std::fs::remove_file(path).expect("failed to remove known_hosts file");
}
