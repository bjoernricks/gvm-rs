// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#[cfg(all(feature = "async-tokio", feature = "ssh-async"))]
mod async_mode;
#[cfg(not(all(feature = "async-tokio", feature = "ssh-async")))]
mod sync_mode;

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use dotenvy::dotenv;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[derive(Debug, Parser)]
#[command(name = "gvm-cli")]
#[command(about = "CLI for the Greenbone Vulnerability Management API")]
struct CliOptions {
    #[arg(long, env = "GVM_GMP_USERNAME")]
    gmp_username: String,

    #[arg(long, env = "GVM_GMP_PASSWORD")]
    gmp_password: String,

    #[arg(long, env = "GVM_CONNECTION_TIMEOUT")]
    timeout: Option<u64>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "socket")]
    Socket(SocketCommand),

    #[command(name = "ssh")]
    Ssh(SshCommand),
}

#[derive(Debug, Args)]
pub struct SocketCommand {
    #[arg(
        long,
        env = "GVM_SOCKET_PATH",
        default_value = "/tmp/gvm/gvmd/gvmd.sock"
    )]
    socket_path: PathBuf,
}

#[derive(Debug, Args)]
pub struct SshCommand {
    #[arg(long, env = "GVM_SSH_HOSTNAME", default_value = "localhost")]
    ssh_hostname: String,

    #[arg(long, env = "GVM_SSH_PORT", default_value_t = 22)]
    ssh_port: u16,

    #[arg(long, env = "GVM_SSH_USERNAME", default_value = "gmp")]
    ssh_username: String,

    #[arg(long, env = "GVM_SSH_PASSWORD")]
    ssh_password: Option<String>,

    #[arg(long, env = "GVM_SSH_AUTO_ACCEPT_HOST", default_value_t = false)]
    ssh_auto_accept_host: bool,
}

fn init_logging() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("GVM_LOG"))
        .init();
}

#[cfg(not(all(feature = "async-tokio", feature = "ssh-async")))]
fn main() {
    let _ = dotenv();
    init_logging();
    let options = CliOptions::parse();
    sync_mode::run(&options);
}

#[cfg(all(feature = "async-tokio", feature = "ssh-async"))]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _ = dotenv();
    init_logging();
    let options = CliOptions::parse();
    async_mode::run(&options).await;
}
