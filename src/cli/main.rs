// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#[cfg(feature = "async-tokio")]
mod async_mode;
#[cfg(not(feature = "async-tokio"))]
mod sync_mode;

use tracing_subscriber::{EnvFilter, fmt, prelude::*};

fn init_logging() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("GVM_LOG"))
        .init();
}

#[cfg(not(feature = "async-tokio"))]
fn main() {
    init_logging();
    sync_mode::run();
}

#[cfg(feature = "async-tokio")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    init_logging();
    async_mode::run().await;
}
