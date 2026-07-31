use std::{
    fmt,
    os::unix::net::UnixStream,
};

use cucumber::World;
use gvm_rs::client::GmpClient;

use crate::config::TestSettings;

#[derive(World)]
#[world(init = Self::new)]
pub struct GvmdWorld {
    pub settings: TestSettings,
    pub client: Option<GmpClient<UnixStream>>,
}

impl GvmdWorld {
    fn new() -> Self {
        let settings =
            TestSettings::load().expect("failed to load GEA BDD test settings");

        Self {
            settings,
            client: None,
        }
    }
}

impl fmt::Debug for GvmdWorld {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GvmdWorld")
            .field("settings", &self.settings)
            .field("client_connected", &self.client.is_some())
            .finish()
    }
}