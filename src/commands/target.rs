// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use chrono::{DateTime, Utc};
use gvm_rs_derive::HasId;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{BoolFromInt, serde_as};
use uuid::Uuid;

use crate::commands::entity::{Entity, HasId, Owner, Permission, UserTags};
use crate::deserialize::{
    define_collection_counts_deserializer, define_unwrap_vec_field, unwrap_and_skip_empty_id,
    unwrap_csv_string, unwrap_optional_csv_string, unwrap_permissions,
};

define_collection_counts_deserializer!(TargetsCounts, "targets", "target_count");

pub mod create_target;
pub mod get_targets;

pub use create_target::CreateTargetRequest;
pub use get_targets::{GetTargetsRequest, GetTargetsResponse};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "alive_test")]
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
