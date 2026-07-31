// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use gvm_rs_derive::HasId;
use serde::Deserialize;
use serde_with::{BoolFromInt, serde_as};
use uuid::Uuid;

use crate::deserialize::unwrap_optional_uuid;

pub trait HasId {
    fn id(&self) -> Option<&Uuid>;

    fn has_id(&self) -> bool {
        self.id().is_some()
    }
}

#[derive(Debug, Deserialize)]
pub struct Permission {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Permissions {
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
    #[serde(rename = "@id", deserialize_with = "unwrap_optional_uuid")]
    pub id: Option<Uuid>,
    pub name: String,
    #[serde_as(as = "BoolFromInt")]
    #[serde(default)]
    pub trash: bool,
    pub permissions: Option<Vec<String>>,
}

impl HasId for Entity {
    fn id(&self) -> Option<&Uuid> {
        self.id.as_ref()
    }
}

#[derive(Debug, Deserialize, HasId)]
pub struct Tag {
    #[serde(rename = "@id")]
    pub id: Uuid,
    pub name: String,
    pub value: String,
    pub comment: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct UserTags {
    pub count: u32,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[cfg(test)]
#[path = "entity_test.rs"]
mod tests;
