// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};
use serde_with::{BoolFromInt, serde_as};

#[derive(Debug, Serialize)]
struct Credentials {
    username: String,
    password: String,
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename = "authenticate")]
pub struct AuthenticateRequest {
    credentials: Credentials,
    #[serde(rename = "@token", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<BoolFromInt>")]
    token: Option<bool>,
}

impl AuthenticateRequest {
    pub fn new(username: &str, password: &str) -> Self {
        AuthenticateRequest {
            credentials: Credentials {
                username: username.to_string(),
                password: password.to_string(),
            },
            token: None,
        }
    }

    pub fn with_token(mut self) -> Self {
        self.token = Some(true);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename = "authenticate_response")]
pub struct AuthenticateResponse {
    #[serde(rename = "@status")]
    pub status: u16,
    #[serde(rename = "@status_text")]
    pub status_text: String,
    pub role: String,
    pub timezone: String,
    pub token: Option<String>,
}
