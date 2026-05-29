// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;
use serde_with::{BoolFromInt, serde_as};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Permission {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Permissions {
    #[serde(default)]
    pub permission: Vec<Permission>,
}

#[derive(Debug, Deserialize)]
pub struct Owner {
    pub name: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct Entity {
    // should be Option<Uuid>, but we can't parse empty string or 0 as UUID
    #[serde(rename = "@id")]
    pub id: String,
    pub name: String,
    #[serde_as(as = "BoolFromInt")]
    pub trash: bool,
    #[serde(default)]
    pub permissions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    #[serde(rename = "@id")]
    pub id: Uuid,
}

#[derive(Debug, Deserialize, Default)]
pub struct UserTags {
    pub count: u32,
    #[serde(default)]
    pub tags: Vec<Tag>,
}
