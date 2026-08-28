// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Serialize, Serializer};
use serde_with::{BoolFromInt, serde_as};
use uuid::Uuid;

use crate::serialize::{serialize_csv, serialize_optional_csv};

#[derive(Debug, Serialize)]
pub struct IdReference {
    #[serde(rename = "@id")]
    pub id: Uuid,
}

#[derive(Serialize)]
struct AliveTests<'a> {
    #[serde(rename = "alive_test")]
    alive_test: &'a [super::AliveTest],
}

fn serialize_alive_tests<S>(
    alive_tests: &[super::AliveTest],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    AliveTests {
        alive_test: alive_tests,
    }
    .serialize(serializer)
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename = "create_target")]
pub struct CreateTargetRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(serialize_with = "serialize_csv")]
    pub hosts: Vec<String>,
    #[serde(
        serialize_with = "serialize_optional_csv",
        skip_serializing_if = "Option::is_none"
    )]
    pub exclude_hosts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_credential: Option<IdReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_credential: Option<IdReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esxi_credential: Option<IdReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub krb5_credential: Option<IdReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snmp_credential: Option<IdReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_elevate_credential: Option<IdReference>,
    #[serde(
        rename = "port_range",
        serialize_with = "serialize_optional_csv",
        skip_serializing_if = "Option::is_none"
    )]
    pub port_ranges: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_list: Option<IdReference>,
    #[serde(serialize_with = "serialize_alive_tests")]
    pub alive_tests: Vec<super::AliveTest>,
    #[serde_as(as = "BoolFromInt")]
    pub reverse_lookup_only: bool,
    #[serde_as(as = "BoolFromInt")]
    pub reverse_lookup_unify: bool,
    #[serde_as(as = "BoolFromInt")]
    pub allow_simultaneous_ips: bool,
}

impl CreateTargetRequest {
    pub fn from_port_list(name: &str, port_list: Uuid) -> Self {
        Self {
            name: name.to_string(),
            comment: None,
            hosts: Vec::new(),
            exclude_hosts: None,
            ssh_credential: None,
            smb_credential: None,
            esxi_credential: None,
            krb5_credential: None,
            snmp_credential: None,
            ssh_elevate_credential: None,
            port_ranges: None,
            port_list: Some(IdReference { id: port_list }),
            alive_tests: Vec::new(),
            reverse_lookup_only: false,
            reverse_lookup_unify: false,
            allow_simultaneous_ips: false,
        }
    }

    pub fn from_port_ranges<I, S>(name: &str, port_ranges: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            name: name.to_string(),
            comment: None,
            hosts: Vec::new(),
            exclude_hosts: None,
            ssh_credential: None,
            smb_credential: None,
            esxi_credential: None,
            krb5_credential: None,
            snmp_credential: None,
            ssh_elevate_credential: None,
            port_ranges: Some(port_ranges.into_iter().map(Into::into).collect()),
            port_list: None,
            alive_tests: Vec::new(),
            reverse_lookup_only: false,
            reverse_lookup_unify: false,
            allow_simultaneous_ips: false,
        }
    }

    pub fn with_comment(mut self, comment: &str) -> Self {
        self.comment = Some(comment.to_string());
        self
    }

    pub fn with_hosts<I, S>(mut self, hosts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.hosts = hosts.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_exclude_hosts<I, S>(mut self, exclude_hosts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.exclude_hosts = Some(exclude_hosts.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_ssh_credential(mut self, ssh_credential: Uuid) -> Self {
        self.ssh_credential = Some(IdReference { id: ssh_credential });
        self
    }

    pub fn with_smb_credential(mut self, smb_credential: Uuid) -> Self {
        self.smb_credential = Some(IdReference { id: smb_credential });
        self
    }

    pub fn with_esxi_credential(mut self, esxi_credential: Uuid) -> Self {
        self.esxi_credential = Some(IdReference {
            id: esxi_credential,
        });
        self
    }

    pub fn with_krb5_credential(mut self, krb5_credential: Uuid) -> Self {
        self.krb5_credential = Some(IdReference {
            id: krb5_credential,
        });
        self
    }

    pub fn with_snmp_credential(mut self, snmp_credential: Uuid) -> Self {
        self.snmp_credential = Some(IdReference {
            id: snmp_credential,
        });
        self
    }

    pub fn with_ssh_elevate_credential(mut self, ssh_elevate_credential: Uuid) -> Self {
        self.ssh_elevate_credential = Some(IdReference {
            id: ssh_elevate_credential,
        });
        self
    }

    pub fn with_alive_tests(mut self, alive_tests: Vec<super::AliveTest>) -> Self {
        self.alive_tests = alive_tests;
        self
    }

    pub fn with_reverse_lookup_only(mut self, reverse_lookup_only: bool) -> Self {
        self.reverse_lookup_only = reverse_lookup_only;
        self
    }

    pub fn with_reverse_lookup_unify(mut self, reverse_lookup_unify: bool) -> Self {
        self.reverse_lookup_unify = reverse_lookup_unify;
        self
    }

    pub fn with_allow_simultaneous_ips(mut self, allow_simultaneous_ips: bool) -> Self {
        self.allow_simultaneous_ips = allow_simultaneous_ips;
        self
    }
}

#[cfg(test)]
#[path = "create_target_test.rs"]
mod tests;
