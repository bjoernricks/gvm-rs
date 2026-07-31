// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use chrono::{DateTime, Utc};
use gvm_rs_derive::HasId;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{BoolFromInt, serde_as};
use uuid::Uuid;

use crate::commands::entity::{Entity, Owner, Permission, UserTags};
use crate::commands::entity::{HasId, QueryFilter};
use crate::deserialize::{
    define_unwrap_vec_field, unwrap_and_skip_empty_id, unwrap_csv_string,
    unwrap_optional_csv_string, unwrap_permissions,
};

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
    #[serde(rename = "filters")]
    pub filter: QueryFilter,
}

#[derive(Debug, Deserialize)]
pub enum AliveTest {
    #[serde(rename = "Scan Config Default")]
    ScanConfigDefault,
    #[serde(rename = "Consider Alive")]
    ConsiderAlive,
    #[serde(rename = "ICMP Ping")]
    IcmpPing,
    #[serde(rename = "TCP-ACK Service Ping")]
    TcpAckServicePing,
    #[serde(rename = "TCP-SYN Service Ping")]
    TcpSynServicePing,
    #[serde(rename = "ARP Ping")]
    ArpPing,
    #[serde(rename = "Host Discovery IPv6")]
    HostDiscoveryIpv6,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
struct AliveTestField {
    #[serde(rename = "$text")]
    alive_test: AliveTest,
}

#[derive(Debug, Deserialize)]
struct AliveTests {
    pub alive_test: Vec<AliveTestField>,
}

fn unwrap_alive_tests<'de, D>(deserializer: D) -> Result<Vec<AliveTest>, D::Error>
where
    D: Deserializer<'de>,
{
    let alive_tests = AliveTests::deserialize(deserializer)?;
    Ok(alive_tests
        .alive_test
        .into_iter()
        .map(|t| t.alive_test)
        .collect())
}

define_unwrap_vec_field!(unwrap_tasks, Tasks, task, Entity);

#[serde_as]
#[derive(Debug, Deserialize, HasId)]
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
    #[serde(deserialize_with = "unwrap_and_skip_empty_id")]
    pub ssh_credential: Vec<Entity>,
    #[serde(deserialize_with = "unwrap_and_skip_empty_id")]
    pub smb_credential: Vec<Entity>,
    #[serde(deserialize_with = "unwrap_and_skip_empty_id")]
    pub esxi_credential: Vec<Entity>,
    #[serde(deserialize_with = "unwrap_and_skip_empty_id")]
    pub krb5_credential: Vec<Entity>,
    #[serde(deserialize_with = "unwrap_and_skip_empty_id")]
    pub snmp_credential: Vec<Entity>,
    #[serde(deserialize_with = "unwrap_and_skip_empty_id")]
    pub ssh_elevate_credential: Vec<Entity>,
    // port_range may be not available if only tasks are requested, so we need to use Option
    #[serde(deserialize_with = "unwrap_optional_csv_string", default)]
    pub port_range: Option<Vec<String>>,
    pub port_list: Entity,
    #[serde(deserialize_with = "unwrap_alive_tests")]
    pub alive_tests: Vec<AliveTest>,
    #[serde_as(as = "BoolFromInt")]
    pub reverse_lookup_only: bool,
    #[serde_as(as = "BoolFromInt")]
    pub reverse_lookup_unify: bool,
    #[serde_as(as = "BoolFromInt")]
    pub allow_simultaneous_ips: bool,
    #[serde(default, deserialize_with = "unwrap_tasks")]
    pub tasks: Vec<Entity>,
}

#[cfg(test)]
#[path = "target_test.rs"]
mod tests;
