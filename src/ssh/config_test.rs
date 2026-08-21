// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use super::{DEFAULT_SSH_HOSTNAME, DEFAULT_SSH_PORT, DEFAULT_SSH_USERNAME, SshConfig};
use std::path::PathBuf;
use std::time::Duration;

#[test]
fn new_config_uses_supplied_connection_settings() {
    let config = SshConfig::new("scanner.example", 2222, "scanner");

    assert_eq!(config.hostname, "scanner.example");
    assert_eq!(config.port, 2222);
    assert_eq!(config.username, "scanner");
    assert!(config.password.is_none());
    assert!(config.identity_file.is_none());
    assert!(config.known_hosts_file.is_none());
    assert!(!config.auto_accept_host);
    assert!(config.timeout.is_none());
}

#[test]
fn default_config_uses_standard_ssh_settings() {
    let config = SshConfig::default();

    assert_eq!(config.hostname, DEFAULT_SSH_HOSTNAME);
    assert_eq!(config.port, DEFAULT_SSH_PORT);
    assert_eq!(config.username, DEFAULT_SSH_USERNAME);
    assert!(config.password.is_none());
    assert!(config.identity_file.is_none());
    assert!(
        config
            .known_hosts_file
            .as_ref()
            .is_some_and(|path| path.ends_with(".ssh/known_hosts"))
    );
    assert!(!config.auto_accept_host);
    assert_eq!(config.timeout, Some(Duration::from_secs(60)));
}

#[test]
fn config_builders_set_connection_options() {
    let identity_file = PathBuf::from("/tmp/id_ed25519");
    let known_hosts_file = PathBuf::from("/tmp/known_hosts");
    let config = SshConfig::new("scanner.example", 2222, "scanner")
        .with_password("secret".to_string())
        .with_identity_file(identity_file.clone())
        .with_known_hosts_file(known_hosts_file.clone())
        .with_auto_accept_host(true)
        .with_timeout(Duration::from_secs(15));

    assert_eq!(config.password.as_deref(), Some("secret"));
    assert_eq!(config.identity_file, Some(identity_file));
    assert_eq!(config.known_hosts_file, Some(known_hosts_file));
    assert!(config.auto_accept_host);
    assert_eq!(config.timeout, Some(Duration::from_secs(15)));
}

#[test]
fn default_known_hosts_builder_uses_ssh_directory() {
    let config = SshConfig::new("scanner.example", 2222, "scanner").with_default_known_hosts_file();

    assert!(
        config
            .known_hosts_file
            .as_ref()
            .is_some_and(|path| path.ends_with(".ssh/known_hosts"))
    );
}
