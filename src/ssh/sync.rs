// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::path::PathBuf;
use std::time::Duration;

use super::config::{DEFAULT_SSH_PORT, SshConfig};

/// Bidirectional SSH exec channel that owns both the session and the channel.
///
/// ssh2::Channel holds a raw *mut LIBSSH2_SESSION pointer internally;
/// declaring `channel` before `session` ensures it is dropped first.
pub struct SshStream {
    channel: ssh2::Channel,
    #[allow(dead_code)]
    session: ssh2::Session,
}

impl SshStream {
    fn new(session: ssh2::Session, channel: ssh2::Channel) -> Self {
        SshStream { channel, session }
    }
}

impl Read for SshStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.channel.read(buf)
    }
}

impl Write for SshStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.channel.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.channel.flush()
    }
}

pub(crate) fn connect(config: &SshConfig) -> Result<SshStream, crate::errors::Error> {
    let tcp = tcp_connect(&config.hostname, config.port, config.timeout)?;

    let mut session = ssh2::Session::new().map_err(crate::errors::Error::SshError)?;
    session.set_tcp_stream(tcp);
    session
        .handshake()
        .map_err(crate::errors::Error::SshError)?;

    check_known_hosts(
        &session,
        &config.hostname,
        config.port,
        &config.known_hosts_file,
        config.auto_accept_host,
    )?;

    if let Some(identity_file) = &config.identity_file {
        session
            .userauth_pubkey_file(&config.username, None, identity_file, None)
            .map_err(crate::errors::Error::SshError)?;
    } else if let Some(password) = &config.password {
        session
            .userauth_password(&config.username, password)
            .map_err(crate::errors::Error::SshError)?;
    } else {
        return Err(crate::errors::Error::ConnectionError(io::Error::new(
            io::ErrorKind::InvalidInput,
            "either password or identity_file must be provided for SSH authentication",
        )));
    }

    let mut channel = session
        .channel_session()
        .map_err(crate::errors::Error::SshError)?;
    channel.exec("").map_err(crate::errors::Error::SshError)?;

    Ok(SshStream::new(session, channel))
}

fn tcp_connect(
    hostname: &str,
    port: u16,
    timeout: Option<Duration>,
) -> Result<TcpStream, crate::errors::Error> {
    if let Some(t) = timeout {
        let addr = format!("{hostname}:{port}");
        let socket_addr = addr
            .to_socket_addrs()
            .map_err(crate::errors::Error::ConnectionError)?
            .next()
            .ok_or_else(|| {
                crate::errors::Error::ConnectionError(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "could not resolve hostname",
                ))
            })?;
        let tcp = TcpStream::connect_timeout(&socket_addr, t)
            .map_err(crate::errors::Error::ConnectionError)?;
        tcp.set_read_timeout(Some(t))
            .map_err(crate::errors::Error::ConnectionError)?;
        tcp.set_write_timeout(Some(t))
            .map_err(crate::errors::Error::ConnectionError)?;
        Ok(tcp)
    } else {
        TcpStream::connect((hostname, port)).map_err(crate::errors::Error::ConnectionError)
    }
}

fn check_known_hosts(
    session: &ssh2::Session,
    hostname: &str,
    port: u16,
    known_hosts_file: &Option<PathBuf>,
    auto_accept: bool,
) -> Result<(), crate::errors::Error> {
    let mut known_hosts = session
        .known_hosts()
        .map_err(crate::errors::Error::SshError)?;

    if let Some(known_hosts_file) = known_hosts_file
        && known_hosts_file.exists()
    {
        known_hosts
            .read_file(known_hosts_file, ssh2::KnownHostFileKind::OpenSSH)
            .map_err(crate::errors::Error::SshError)?;
    }

    let (key, key_type) = session.host_key().ok_or_else(|| {
        crate::errors::Error::ConnectionError(io::Error::other("no host key received from server"))
    })?;

    match known_hosts.check_port(hostname, port, key) {
        ssh2::CheckResult::Match => Ok(()),
        ssh2::CheckResult::NotFound => {
            if auto_accept {
                let host_entry = if port == DEFAULT_SSH_PORT {
                    hostname.to_string()
                } else {
                    format!("[{hostname}]:{port}")
                };
                known_hosts
                    .add(&host_entry, key, "", key_type.into())
                    .map_err(crate::errors::Error::SshError)?;
                if let Some(known_hosts_file) = known_hosts_file {
                    if let Some(parent) = known_hosts_file.parent() {
                        std::fs::create_dir_all(parent)
                            .map_err(crate::errors::Error::ConnectionError)?;
                    }
                    known_hosts
                        .write_file(known_hosts_file, ssh2::KnownHostFileKind::OpenSSH)
                        .map_err(crate::errors::Error::SshError)?;
                }
                Ok(())
            } else {
                Err(crate::errors::Error::ConnectionError(io::Error::new(
                    io::ErrorKind::ConnectionRefused,
                    format!("host '{hostname}' not found in known_hosts"),
                )))
            }
        }
        ssh2::CheckResult::Mismatch => Err(crate::errors::Error::ConnectionError(io::Error::new(
            io::ErrorKind::ConnectionRefused,
            format!("host key mismatch for '{hostname}' — possible MITM attack"),
        ))),
        ssh2::CheckResult::Failure => Err(crate::errors::Error::ConnectionError(io::Error::other(
            "known_hosts check failed",
        ))),
    }
}

#[cfg(test)]
#[path = "sync_test.rs"]
mod tests;
