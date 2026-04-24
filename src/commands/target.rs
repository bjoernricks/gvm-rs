// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};
use serde_with::{BoolFromInt, serde_as};

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename = "get_targets")]
pub struct GetTargetsRequest {
    #[serde(rename = "@details", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<BoolFromInt>")]
    details: Option<bool>,
    #[serde(rename = "@filt_id", skip_serializing_if = "Option::is_none")]
    filter_id: Option<String>,
    #[serde(rename = "@trash", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<BoolFromInt>")]
    trash: Option<bool>,
    #[serde(rename = "@tasks", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<BoolFromInt>")]
    tasks: Option<bool>,
}

impl GetTargetsRequest {
    pub fn new(
        details: Option<bool>,
        filter_id: Option<String>,
        trash: Option<bool>,
        tasks: Option<bool>,
    ) -> Self {
        GetTargetsRequest {
            details,
            filter_id,
            trash,
            tasks,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename = "get_targets_response")]
pub struct GetTargetsResponse {}
