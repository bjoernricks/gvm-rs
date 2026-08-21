// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod config;

pub use config::{DEFAULT_SSH_HOSTNAME, DEFAULT_SSH_PORT, DEFAULT_SSH_USERNAME, SshConfig};

#[cfg(feature = "ssh")]
mod sync;
#[cfg(feature = "ssh")]
pub use sync::SshStream;
#[cfg(feature = "ssh")]
pub(crate) use sync::connect;

#[cfg(feature = "ssh-async")]
mod r#async;
#[cfg(feature = "ssh-async")]
pub use r#async::SshAsyncStream;
#[cfg(feature = "ssh-async")]
pub(crate) use r#async::connect_async;
