// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#[cfg(feature = "async-tokio")]
mod async_mode;
#[cfg(not(feature = "async-tokio"))]
mod sync_mode;

use std::path::PathBuf;

use clap::Parser;
use dotenvy::dotenv;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[derive(Debug, Parser)]
#[command(name = "gvm-cli")]
#[command(about = "CLI for the Greenbone Vulnerability Management API")]
struct CliOptions {
    #[arg(
        long,
        env = "GVM_SOCKET_PATH",
        default_value = "/tmp/gvm/gvmd/gvmd.sock"
    )]
    socket_path: PathBuf,

    #[arg(long, env = "GVM_USERNAME")]
    username: String,

    #[arg(long, env = "GVM_PASSWORD")]
    password: String,
}

fn init_logging() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("GVM_LOG"))
        .init();
}

#[cfg(not(feature = "async-tokio"))]
fn main() {
    let _ = dotenv();
    init_logging();
    let options = CliOptions::parse();
    sync_mode::run(&options);
}

#[cfg(feature = "async-tokio")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _ = dotenv();
    init_logging();
    let options = CliOptions::parse();
    async_mode::run(&options).await;
}
