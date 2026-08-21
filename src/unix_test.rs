// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use super::{DEFAULT_SOCKET_PATH, UnixSocketConfig};
use std::path::PathBuf;
use std::time::Duration;

#[test]
fn new_config_uses_supplied_socket_path() {
    let config = UnixSocketConfig::new("/tmp/gvmd.sock");

    assert_eq!(config.socket_path, PathBuf::from("/tmp/gvmd.sock"));
    assert!(config.timeout.is_none());
}

#[test]
fn default_config_uses_default_socket_path_without_timeout() {
    let config = UnixSocketConfig::default();

    assert_eq!(config.socket_path, PathBuf::from(DEFAULT_SOCKET_PATH));
    assert!(config.timeout.is_none());
}

#[test]
fn timeout_builder_sets_per_io_timeout() {
    let config = UnixSocketConfig::new("/tmp/gvmd.sock").with_timeout(Duration::from_secs(15));

    assert_eq!(config.socket_path, PathBuf::from("/tmp/gvmd.sock"));
    assert_eq!(config.timeout, Some(Duration::from_secs(15)));
}
