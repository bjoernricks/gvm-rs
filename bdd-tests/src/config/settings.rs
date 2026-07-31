use std::{fmt, path::PathBuf};

use config::{ConfigBuilder, builder::DefaultState};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Clone)]
pub struct TestSettings {
    pub socket_path: PathBuf,
    pub username: String,
    pub password: String,
    pub log_level: String,
}

#[derive(Debug, Clone, Deserialize)]
struct RawTestSettings {
    socket_path: PathBuf,
    username: String,
    password: String,
    log_level: String,
}

impl TestSettings {
    pub fn load() -> Result<Self, config::ConfigError> {
        let _ = dotenvy::dotenv();

        let config = Self::config_builder()?
            .add_source(config::Environment::with_prefix("GVMD_TEST").prefix_separator("_"))
            .build()?;

        let raw = config.try_deserialize::<RawTestSettings>()?;

        Self::from_raw(raw)
    }

    fn config_builder() -> Result<ConfigBuilder<DefaultState>, config::ConfigError> {
        config::Config::builder()
            .set_default("socket_path", "/run/gvmd/gvmd.sock")?
            .set_default("username", "admin")?
            .set_default("password", "admin")?
            .set_default("log_level", "info")
    }

    fn from_raw(raw: RawTestSettings) -> Result<Self, config::ConfigError> {
        if raw.socket_path.as_os_str().is_empty() {
            return Err(config::ConfigError::Message(
                "socket_path must not be empty".to_string(),
            ));
        }

        if raw.username.trim().is_empty() {
            return Err(config::ConfigError::Message(
                "username must not be empty".to_string(),
            ));
        }

        if raw.password.is_empty() {
            return Err(config::ConfigError::Message(
                "password must not be empty".to_string(),
            ));
        }

        if raw.log_level.trim().is_empty() {
            return Err(config::ConfigError::Message(
                "log_level must not be empty".to_string(),
            ));
        }

        Ok(Self {
            socket_path: raw.socket_path,
            username: raw.username,
            password: raw.password,
            log_level: raw.log_level,
        })
    }
}

impl fmt::Debug for TestSettings {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TestSettings")
            .field("socket_path", &self.socket_path)
            .field("username", &self.username)
            .field("password", &"[REDACTED]")
            .field("log_level", &self.log_level)
            .finish()
    }
}
