// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Serializer;
use std::fmt::Display;

pub(crate) fn serialize_csv<S, T>(values: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Display,
{
    let value = values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",");
    serializer.serialize_str(&value)
}

pub(crate) fn serialize_optional_csv<S, T>(
    values: &Option<Vec<T>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Display,
{
    match values {
        Some(values) => serialize_csv(values, serializer),
        None => serializer.serialize_none(),
    }
}

#[cfg(test)]
#[path = "serialize_test.rs"]
mod tests;
