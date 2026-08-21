// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(feature = "async-tokio")]
pub mod async_client;
pub mod client;
pub mod commands;
pub mod deserialize;
pub mod errors;
#[cfg(any(feature = "ssh", feature = "ssh-async"))]
pub mod ssh;
pub mod unix;
