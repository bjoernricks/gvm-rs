// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};
use serde_with::{BoolFromInt, serde_as};

use crate::commands::entity::QueryFilter;

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
    pub fn new() -> Self {
        GetTargetsRequest {
            details: None,
            filter_id: None,
            trash: None,
            tasks: None,
        }
    }

    pub fn with_details(mut self) -> Self {
        self.details = Some(true);
        self
    }

    pub fn with_filter_id(mut self, filter_id: &str) -> Self {
        self.filter_id = Some(filter_id.to_string());
        self
    }

    pub fn with_trash(mut self) -> Self {
        self.trash = Some(true);
        self
    }

    pub fn with_tasks(mut self) -> Self {
        self.tasks = Some(true);
        self
    }
}

impl Default for GetTargetsRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename = "get_targets_response")]
pub struct GetTargetsResponse {
    #[serde(rename = "@status")]
    pub status: u16,
    #[serde(rename = "@status_text")]
    pub status_text: String,
    #[serde(rename = "target", default)]
    pub targets: Vec<super::Target>,
    #[serde(rename = "filters")]
    pub filter: QueryFilter,
    #[serde(flatten)]
    pub counts: super::TargetsCounts,
}

#[cfg(test)]
#[path = "get_targets_test.rs"]
mod tests;
