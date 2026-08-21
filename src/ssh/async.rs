// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io;
use std::net::ToSocketAddrs;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use russh::client;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

use super::config::{DEFAULT_SSH_PORT, SshConfig};

struct SshHandler {
    hostname: String,
    port: u16,
    known_hosts_file: Option<PathBuf>,
    auto_accept_host: bool,
}

impl client::Handler for SshHandler {
    type Error = crate::errors::Error;

    async fn check_server_key(
        &mut self,
        server_public_key: &russh::keys::PublicKey,
    ) -> Result<bool, Self::Error> {
        check_server_key_against_file(
            &self.hostname,
            self.port,
            server_public_key,
            self.known_hosts_file.clone(),
            self.auto_accept_host,
        )
    }
}

fn check_server_key_against_file(
    hostname: &str,
    port: u16,
    server_key: &russh::keys::PublicKey,
    known_hosts_file: Option<PathBuf>,
    auto_accept: bool,
) -> Result<bool, crate::errors::Error> {
    let host_entry = if port == DEFAULT_SSH_PORT {
        hostname.to_string()
    } else {
        format!("[{hostname}]:{port}")
    };

    let known_hosts_file = match known_hosts_file {
        Some(path) => path,
        None => {
            return Err(crate::errors::Error::ConnectionError(io::Error::new(
                io::ErrorKind::NotFound,
                "no known_hosts file specified",
            )));
        }
    };

    let content = match std::fs::read_to_string(&known_hosts_file) {
        Ok(c) => c,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            return handle_unknown_host(&host_entry, server_key, &known_hosts_file, auto_accept);
        }
        Err(e) => return Err(crate::errors::Error::ConnectionError(e)),
    };

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.splitn(4, ' ');
        let hosts = parts.next().unwrap_or("");
        let key_type = parts.next().unwrap_or("");
        let key_b64 = parts.next().unwrap_or("");

        if !hosts.split(',').any(|h| h == host_entry) {
            continue;
        }

        let entry = format!("{key_type} {key_b64}");
        match russh::keys::PublicKey::from_openssh(&entry) {
            Ok(known_key) if known_key == *server_key => return Ok(true),
            Ok(_) => {
                return Err(crate::errors::Error::ConnectionError(io::Error::new(
                    io::ErrorKind::ConnectionRefused,
                    format!("host key mismatch for '{hostname}' — possible MITM attack"),
                )));
            }
            Err(_) => continue,
        }
    }

    handle_unknown_host(&host_entry, server_key, &known_hosts_file, auto_accept)
}

fn handle_unknown_host(
    host_entry: &str,
    server_key: &russh::keys::PublicKey,
    known_hosts_file: &Path,
    auto_accept: bool,
) -> Result<bool, crate::errors::Error> {
    if !auto_accept {
        return Ok(false);
    }

    let key_line = server_key.to_string();
    if let Some(parent) = known_hosts_file.parent() {
        std::fs::create_dir_all(parent).map_err(crate::errors::Error::ConnectionError)?;
    }

    use std::io::Write as _;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(known_hosts_file)
        .map_err(crate::errors::Error::ConnectionError)?;
    writeln!(file, "{host_entry} {key_line}").map_err(crate::errors::Error::ConnectionError)?;

    Ok(true)
}

/// Owns the russh session handle and the channel stream together.
pub struct SshAsyncStream {
    stream: russh::ChannelStream<client::Msg>,
    _handle: client::Handle<SshHandler>,
}

impl AsyncRead for SshAsyncStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.stream).poll_read(cx, buf)
    }
}

impl AsyncWrite for SshAsyncStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.stream).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.stream).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.stream).poll_shutdown(cx)
    }
}

pub(crate) async fn connect_async(
    config: &SshConfig,
) -> Result<SshAsyncStream, crate::errors::Error> {
    let russh_config = Arc::new(client::Config::default());

    let handler = SshHandler {
        hostname: config.hostname.clone(),
        port: config.port,
        known_hosts_file: config.known_hosts_file.clone(),
        auto_accept_host: config.auto_accept_host,
    };

    let address = format!("{}:{}", config.hostname, config.port)
        .to_socket_addrs()
        .map_err(crate::errors::Error::ConnectionError)?
        .next()
        .ok_or_else(|| {
            crate::errors::Error::ConnectionError(io::Error::new(
                io::ErrorKind::InvalidInput,
                "could not resolve hostname",
            ))
        })?;
    let socket = if let Some(timeout) = config.timeout {
        tokio::time::timeout(timeout, tokio::net::TcpStream::connect(address))
            .await
            .map_err(|_| {
                crate::errors::Error::ConnectionError(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "SSH TCP connection timed out",
                ))
            })??
    } else {
        tokio::net::TcpStream::connect(address).await?
    };
    let mut handle = client::connect_stream(russh_config, socket, handler).await?;

    let auth_result = if let Some(identity_file) = &config.identity_file {
        let key = russh::keys::PrivateKey::read_openssh_file(identity_file)
            .map_err(|e| crate::errors::Error::ConnectionError(io::Error::other(e)))?;
        handle
            .authenticate_publickey(
                &config.username,
                russh::keys::PrivateKeyWithHashAlg::new(
                    Arc::new(key),
                    handle.best_supported_rsa_hash().await?.flatten(),
                ),
            )
            .await?
    } else if let Some(password) = &config.password {
        handle
            .authenticate_password(&config.username, password)
            .await?
    } else {
        return Err(crate::errors::Error::ConnectionError(io::Error::new(
            io::ErrorKind::InvalidInput,
            "either password or identity_file must be provided for SSH authentication",
        )));
    };

    if !auth_result.success() {
        return Err(crate::errors::Error::ConnectionError(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "SSH authentication failed",
        )));
    }

    let channel = handle.channel_open_session().await?;
    channel.exec(true, "").await?;

    Ok(SshAsyncStream {
        stream: channel.into_stream(),
        _handle: handle,
    })
}

#[cfg(test)]
#[path = "async_test.rs"]
mod tests;
