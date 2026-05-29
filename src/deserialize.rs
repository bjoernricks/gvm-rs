// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Deserializer};

use crate::commands::entity::{Permission, Permissions};

pub fn unwrap_csv_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split(',').map(|s| s.trim().to_string()).collect())
}

pub fn unwrap_permissions<'de, D>(deserializer: D) -> Result<Vec<Permission>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Permissions::deserialize(deserializer)?.permission)
}
