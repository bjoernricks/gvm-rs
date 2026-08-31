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
    define_collection_counts_deserializer, define_unwrap_vec_field, unwrap_permissions,
};

define_collection_counts_deserializer!(CredentialsCounts, "credentials", "credential_count");

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CredentialFormat {
    Key,
    Rpm,
    Deb,
    Exe,
    Pem,
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename = "get_credentials")]
pub struct GetCredentialsRequest {
    #[serde(rename = "@credential_id", skip_serializing_if = "Option::is_none")]
    credential_id: Option<String>,
    #[serde(rename = "@filter", skip_serializing_if = "Option::is_none")]
    filter: Option<String>,
    #[serde(rename = "@filt_id", skip_serializing_if = "Option::is_none")]
    filter_id: Option<String>,
    #[serde(rename = "@details", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<BoolFromInt>")]
    details: Option<bool>,
    #[serde(rename = "@scanners", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<BoolFromInt>")]
    scanners: Option<bool>,
    #[serde(rename = "@trash", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<BoolFromInt>")]
    trash: Option<bool>,
    #[serde(rename = "@targets", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<BoolFromInt>")]
    targets: Option<bool>,
    #[serde(rename = "@oci_image_targets", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<BoolFromInt>")]
    oci_image_targets: Option<bool>,
    #[serde(rename = "@format", skip_serializing_if = "Option::is_none")]
    format: Option<CredentialFormat>,
}

impl GetCredentialsRequest {
    pub fn new() -> Self {
        GetCredentialsRequest {
            credential_id: None,
            filter: None,
            filter_id: None,
            details: None,
            scanners: None,
            trash: None,
            targets: None,
            oci_image_targets: None,
            format: None,
        }
    }

    pub fn with_credential_id(mut self, credential_id: &str) -> Self {
        self.credential_id = Some(credential_id.to_string());
        self
    }

    pub fn with_filter(mut self, filter: &str) -> Self {
        self.filter = Some(filter.to_string());
        self
    }

    pub fn with_filter_id(mut self, filter_id: &str) -> Self {
        self.filter_id = Some(filter_id.to_string());
        self
    }

    pub fn with_details(mut self) -> Self {
        self.details = Some(true);
        self
    }

    pub fn with_scanners(mut self) -> Self {
        self.scanners = Some(true);
        self
    }

    pub fn with_trash(mut self) -> Self {
        self.trash = Some(true);
        self
    }

    pub fn with_targets(mut self) -> Self {
        self.targets = Some(true);
        self
    }

    pub fn with_oci_image_targets(mut self) -> Self {
        self.oci_image_targets = Some(true);
        self
    }

    pub fn with_format(mut self, format: CredentialFormat) -> Self {
        self.format = Some(format);
        self
    }
}

impl Default for GetCredentialsRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename = "get_credentials_response")]
pub struct GetCredentialsResponse {
    #[serde(rename = "@status")]
    pub status: u16,
    #[serde(rename = "@status_text")]
    pub status_text: String,
    pub credential: Vec<Credential>,
    #[serde(rename = "filters")]
    pub filter: QueryFilter,
    #[serde(flatten)]
    pub counts: CredentialsCounts,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CredentialType {
    /// Client certificate
    Cc,
    /// Kerberos 5
    Krb5,
    /// PGP encryption key
    Pgp,
    /// Password only
    Pw,
    /// S/MIME certificate
    Smime,
    /// SNMP
    Snmp,
    /// User name + password
    Up,
    /// User name + SSH key
    Usk,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthAlgorithm {
    Md5,
    Sha1,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrivacyAlgorithm {
    Aes,
    Des,
    #[default]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct Privacy {
    #[serde(default)]
    pub algorithm: PrivacyAlgorithm,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CertificateTimeStatus {
    Expired,
    Inactive,
    Valid,
    #[serde(other)]
    Unknown,
}

fn unwrap_optional_time<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    match s.as_deref().map(str::trim) {
        Some("") | Some("unlimited") | None => Ok(None),
        Some(s) => DateTime::parse_from_rfc3339(s)
            .map(|t| Some(t.with_timezone(&Utc)))
            .map_err(serde::de::Error::custom),
    }
}

#[derive(Debug, Deserialize)]
pub struct CertificateInfo {
    pub time_status: CertificateTimeStatus,
    #[serde(default, deserialize_with = "unwrap_optional_time")]
    pub activation_time: Option<DateTime<Utc>>,
    #[serde(default, deserialize_with = "unwrap_optional_time")]
    pub expiration_time: Option<DateTime<Utc>>,
    pub issuer: String,
    pub md5_fingerprint: String,
    pub sha256_fingerprint: Option<String>,
    pub subject: Option<String>,
    pub serial: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PrivateKeyInfo {
    #[serde(rename = "type")]
    pub key_type: Option<String>,
    pub sha256_hash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PublicKeyInfo {
    pub fingerprint: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PackageFormat {
    Rpm,
    Deb,
    Exe,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    #[serde(rename = "@format")]
    pub format: PackageFormat,
    /// base64 encoded package data
    #[serde(rename = "$text", default)]
    pub data: String,
}

#[derive(Debug, Deserialize)]
struct FormatField {
    #[serde(rename = "$text")]
    format: CredentialFormat,
}

#[derive(Debug, Deserialize)]
struct Formats {
    #[serde(default)]
    format: Vec<FormatField>,
}

fn unwrap_formats<'de, D>(deserializer: D) -> Result<Vec<CredentialFormat>, D::Error>
where
    D: Deserializer<'de>,
{
    let formats = Formats::deserialize(deserializer)?;
    Ok(formats.format.into_iter().map(|f| f.format).collect())
}

define_unwrap_vec_field!(unwrap_scanners, Scanners, scanner, Entity);
define_unwrap_vec_field!(unwrap_targets, Targets, target, Entity);
define_unwrap_vec_field!(
    unwrap_oci_image_targets,
    OciImageTargets,
    oci_image_target,
    Entity
);
define_unwrap_vec_field!(unwrap_kdcs, Kdcs, kdc, String);

#[serde_as]
#[derive(Debug, Deserialize, HasId)]
#[serde(rename = "credential")]
pub struct Credential {
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
    #[serde_as(as = "BoolFromInt")]
    pub allow_insecure: bool,
    pub login: String,
    #[serde(deserialize_with = "unwrap_permissions")]
    pub permissions: Vec<Permission>,
    pub user_tags: Option<UserTags>,
    #[serde(rename = "type")]
    pub credential_type: CredentialType,
    pub full_type: String,
    #[serde(deserialize_with = "unwrap_formats")]
    pub formats: Vec<CredentialFormat>,
    pub auth_algorithm: Option<AuthAlgorithm>,
    pub privacy: Option<Privacy>,
    // certificate_info is only available if details were requested
    pub certificate_info: Option<CertificateInfo>,
    pub private_key_info: Option<PrivateKeyInfo>,
    pub public_key_info: Option<PublicKeyInfo>,
    #[serde(default, deserialize_with = "unwrap_scanners")]
    pub scanners: Vec<Entity>,
    #[serde(default, deserialize_with = "unwrap_targets")]
    pub targets: Vec<Entity>,
    #[serde(default, deserialize_with = "unwrap_oci_image_targets")]
    pub oci_image_targets: Vec<Entity>,
    // at most one of public_key, package and certificate is returned depending
    // on the requested format
    pub public_key: Option<String>,
    pub package: Option<Package>,
    pub certificate: Option<String>,
    /// Deprecated: use [`Credential::kdcs`] instead
    pub kdc: Option<String>,
    #[serde(default, deserialize_with = "unwrap_kdcs")]
    pub kdcs: Vec<String>,
    pub realm: Option<String>,
}

#[cfg(test)]
#[path = "credentials_test.rs"]
mod tests;
