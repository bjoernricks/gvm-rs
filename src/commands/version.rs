// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename = "get_version_response")]
pub struct GetVersionResponse {
    #[serde(rename = "@status")]
    pub status: i32,
    #[serde(rename = "@status_text")]
    pub status_text: String,
    pub version: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename = "get_version")]
pub struct GetVersionRequest;
