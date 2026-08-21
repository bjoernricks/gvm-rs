// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::{Path, PathBuf};
use std::time::Duration;

pub const DEFAULT_SOCKET_PATH: &str = "/var/run/gvmd.sock";

pub struct UnixSocketConfig {
    pub socket_path: PathBuf,
    /// Maximum time a connected socket read or write may remain blocked.
    ///
    /// The timeout is applied separately to each I/O operation. It does not
    /// impose a deadline on connecting or on the complete GMP command.
    pub timeout: Option<Duration>,
}

impl UnixSocketConfig {
    pub fn new<P: AsRef<Path>>(socket_path: P) -> Self {
        Self {
            socket_path: socket_path.as_ref().to_path_buf(),
            timeout: None,
        }
    }

    /// Sets the per-I/O inactivity timeout for a connected socket.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

impl Default for UnixSocketConfig {
    fn default() -> Self {
        Self {
            socket_path: DEFAULT_SOCKET_PATH.into(),
            timeout: None,
        }
    }
}
