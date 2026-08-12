// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Deserializer};

use crate::commands::entity::{HasId, Permission};

#[derive(Debug, Deserialize, Default)]
pub struct CollectionListMeta {
    #[serde(rename = "@start", default)]
    pub first: String,
    #[serde(rename = "@max", default)]
    pub rows: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct TextNode {
    #[serde(rename = "$text", default)]
    pub value: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct CollectionCountMeta {
    #[serde(rename = "$text", default)]
    pub all: String,
    #[serde(default)]
    pub filtered: TextNode,
    #[serde(rename = "page", default)]
    pub length: TextNode,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "@status")]
    pub status: u16,
    #[serde(rename = "@status_text")]
    pub status_text: String,
}

pub fn parse_u32_or_zero(s: &str) -> u32 {
    s.trim().parse().unwrap_or_default()
}

pub fn parse_i32_or_zero(s: &str) -> i32 {
    s.trim().parse().unwrap_or_default()
}

macro_rules! define_collection_counts_deserializer {
    ($name:ident, $list_tag:literal, $count_tag:literal) => {
        #[derive(::serde::Deserialize)]
        struct _CountsMeta {
            #[serde(rename = $list_tag, default)]
            list: crate::deserialize::CollectionListMeta,
            #[serde(rename = $count_tag, default)]
            count: crate::deserialize::CollectionCountMeta,
        }

        #[derive(Debug)]
        pub struct $name {
            counts: crate::commands::entity::CollectionCounts,
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D: ::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                let meta = <_CountsMeta as ::serde::Deserialize>::deserialize(deserializer)?;
                Ok($name {
                    counts: crate::commands::entity::CollectionCounts {
                        first: crate::deserialize::parse_u32_or_zero(&meta.list.first),
                        rows: crate::deserialize::parse_i32_or_zero(&meta.list.rows),
                        all: crate::deserialize::parse_u32_or_zero(&meta.count.all),
                        filtered: crate::deserialize::parse_u32_or_zero(&meta.count.filtered.value),
                        length: crate::deserialize::parse_u32_or_zero(&meta.count.length.value),
                    },
                })
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = crate::commands::entity::CollectionCounts;
            fn deref(&self) -> &Self::Target {
                &self.counts
            }
        }
    };
}

pub(crate) use define_collection_counts_deserializer;

macro_rules! define_unwrap_vec_field {
    ($vis:vis, $func:ident, $wrapper:ident, $field:ident, $item:ty) => {
        #[derive(Debug, ::serde::Deserialize)]
        struct $wrapper {
            #[serde(default)]
            $field: Option<Vec<$item>>,
        }

        $vis fn $func<'de, D>(deserializer: D) -> Result<Vec<$item>, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(
                <$wrapper as ::serde::Deserialize>::deserialize(deserializer)?
                    .$field
                    .unwrap_or_default(),
            )
        }
    };
    ($func:ident, $wrapper:ident, $field:ident, $item:ty) => {
        #[derive(Debug, ::serde::Deserialize)]
        struct $wrapper {
            #[serde(default)]
            $field: Option<Vec<$item>>,
        }

        fn $func<'de, D>(deserializer: D) -> Result<Vec<$item>, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(
                <$wrapper as ::serde::Deserialize>::deserialize(deserializer)?
                    .$field
                    .unwrap_or_default(),
            )
        }
    };
}

pub(crate) use define_unwrap_vec_field;

pub fn unwrap_csv_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split(',').map(|s| s.trim().to_string()).collect())
}

pub fn unwrap_optional_csv_string<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    match s {
        Some(s) => Ok(Some(s.split(',').map(|s| s.trim().to_string()).collect())),
        None => Ok(None),
    }
}

define_unwrap_vec_field!(pub, unwrap_permissions, Permissions, permission, Permission);

pub fn unwrap_and_skip_empty_id<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + HasId,
{
    let entities = Vec::<T>::deserialize(deserializer)?
        .into_iter()
        .filter(|e| !e.has_id())
        .collect();
    Ok(entities)
}

pub fn unwrap_uuid<'de, D>(deserializer: D) -> Result<uuid::Uuid, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.trim().is_empty() || s.trim() == "0" {
        return Ok(uuid::Uuid::nil());
    }
    uuid::Uuid::parse_str(&s).map_err(serde::de::Error::custom)
}

pub fn unwrap_optional_uuid<'de, D>(deserializer: D) -> Result<Option<uuid::Uuid>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    match s {
        Some(s) => {
            if s.trim().is_empty() {
                return Ok(None);
            }
            if s.trim() == "0" {
                return Ok(Some(uuid::Uuid::nil()));
            }
            uuid::Uuid::parse_str(&s)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
        None => Ok(None),
    }
}

#[cfg(test)]
#[path = "deserialize_test.rs"]
mod tests;
