// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{BoolFromInt, serde_as};
use uuid::Uuid;

use crate::commands::entity::{Entity, Owner, Permission, UserTags};
use crate::deserialize::{unwrap_csv_string, unwrap_optional_csv_string, unwrap_permissions};

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
    pub target: Vec<Target>,
}

#[derive(Debug, Deserialize)]
struct AliveTests {
    pub alive_test: Vec<String>,
}

fn unwrap_alive_tests<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(AliveTests::deserialize(deserializer)?.alive_test)
}

#[derive(Debug, Deserialize)]
struct Tasks {
    #[serde(default)]
    pub task: Option<Vec<Entity>>,
}

fn unwrap_tasks<'de, D>(deserializer: D) -> Result<Option<Vec<Entity>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Tasks::deserialize(deserializer)?.task)
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename = "target")]
pub struct Target {
    #[serde(rename = "@id")]
    pub id: Uuid,
    pub name: String,
    pub comment: String,
    pub creation_time: DateTime<Utc>,
    pub modification_time: DateTime<Utc>,
    pub owner: Owner,
    #[serde_as(as = "BoolFromInt")]
    pub writable: bool,
    #[serde_as(as = "BoolFromInt")]
    pub in_use: bool,
    #[serde(deserialize_with = "unwrap_permissions")]
    pub permissions: Vec<Permission>,
    pub user_tags: Option<UserTags>,
    #[serde(deserialize_with = "unwrap_csv_string")]
    pub hosts: Vec<String>,
    #[serde(deserialize_with = "unwrap_csv_string")]
    pub exclude_hosts: Vec<String>,
    pub max_hosts: u32,
    // FIXME skip entities with empty id
    pub ssh_credential: Vec<Entity>,
    // FIXME skip entities with empty id
    pub smb_credential: Vec<Entity>,
    // FIXME skip entities with empty id
    pub esxi_credential: Vec<Entity>,
    // FIXME skip entities with empty id
    pub krb5_credential: Vec<Entity>,
    // FIXME skip entities with empty id
    pub snmp_credential: Vec<Entity>,
    // FIXME skip entities with empty id
    pub ssh_elevate_credential: Vec<Entity>,
    // port_range may be not available if only tasks are requested, so we need to use Option
    #[serde(deserialize_with = "unwrap_optional_csv_string", default)]
    pub port_range: Option<Vec<String>>,
    pub port_list: Entity,
    #[serde(deserialize_with = "unwrap_alive_tests")]
    pub alive_tests: Vec<String>,
    #[serde_as(as = "BoolFromInt")]
    pub reverse_lookup_only: bool,
    #[serde_as(as = "BoolFromInt")]
    pub reverse_lookup_unify: bool,
    #[serde_as(as = "BoolFromInt")]
    pub allow_simultaneous_ips: bool,
    #[serde(default, deserialize_with = "unwrap_tasks")]
    pub tasks: Option<Vec<Entity>>,
}
