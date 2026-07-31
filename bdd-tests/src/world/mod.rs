// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod authentication;

use std::{fmt, os::unix::net::UnixStream};

use cucumber::World;
use gvm_rs::client::GmpClient;

use crate::config::TestSettings;

pub use authentication::AuthenticationState;

#[derive(World)]
#[world(init = Self::new)]
pub struct GvmdWorld {
    pub settings: TestSettings,
    pub client: Option<GmpClient<UnixStream>>,
    pub authentication: AuthenticationState,
}

impl GvmdWorld {
    fn new() -> Self {
        Self {
            settings: TestSettings::load().expect("failed to load gvmd BDD test settings"),
            client: None,
            authentication: AuthenticationState::default(),
        }
    }
}

impl fmt::Debug for GvmdWorld {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GvmdWorld")
            .field("settings", &self.settings)
            .field("client_connected", &self.client.is_some())
            .field("authentication", &self.authentication)
            .finish()
    }
}
