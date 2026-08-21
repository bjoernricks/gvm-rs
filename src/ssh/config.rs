// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::PathBuf;
use std::time::Duration;

pub const DEFAULT_SSH_PORT: u16 = 22;
pub const DEFAULT_SSH_HOSTNAME: &str = "127.0.0.1";
pub const DEFAULT_SSH_USERNAME: &str = "gmp";

/// SSH connection settings, mirroring python-gvm's SSHConnection parameters.
pub struct SshConfig {
    pub hostname: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>,
    /// If set, public-key auth is used; otherwise password auth.
    pub identity_file: Option<PathBuf>,
    pub known_hosts_file: Option<PathBuf>,
    /// `true` auto-accepts and saves unknown host keys (TOFU).
    /// `false` rejects unknown hosts.
    pub auto_accept_host: bool,
    pub timeout: Option<Duration>,
}

impl SshConfig {
    pub fn new(hostname: &str, port: u16, username: &str) -> Self {
        SshConfig {
            hostname: hostname.to_string(),
            port,
            username: username.to_string(),
            password: None,
            identity_file: None,
            known_hosts_file: None,
            auto_accept_host: false,
            timeout: None,
        }
    }

    pub fn with_password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }

    pub fn with_identity_file(mut self, identity_file: PathBuf) -> Self {
        self.identity_file = Some(identity_file);
        self
    }

    pub fn with_known_hosts_file(mut self, known_hosts_file: PathBuf) -> Self {
        self.known_hosts_file = Some(known_hosts_file);
        self
    }

    pub fn with_default_known_hosts_file(mut self) -> Self {
        self.known_hosts_file = Some(default_known_hosts_file());
        self
    }

    pub fn with_auto_accept_host(mut self, auto_accept_host: bool) -> Self {
        self.auto_accept_host = auto_accept_host;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

impl Default for SshConfig {
    fn default() -> Self {
        SshConfig {
            hostname: DEFAULT_SSH_HOSTNAME.to_string(),
            port: DEFAULT_SSH_PORT,
            username: DEFAULT_SSH_USERNAME.to_string(),
            password: None,
            identity_file: None,
            known_hosts_file: Some(default_known_hosts_file()),
            auto_accept_host: false,
            timeout: Some(Duration::from_secs(60)),
        }
    }
}

fn default_known_hosts_file() -> PathBuf {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".ssh")
        .join("known_hosts")
}

#[cfg(test)]
#[path = "config_test.rs"]
mod tests;
