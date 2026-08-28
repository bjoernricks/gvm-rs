// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Serialize;
use std::fmt::Display;

use super::{serialize_csv, serialize_optional_csv};

#[derive(Debug, Serialize)]
#[serde(rename = "wrapper")]
struct Wrapper<T: Display> {
    #[serde(serialize_with = "serialize_csv")]
    value: Vec<T>,
}

#[derive(Debug, Serialize)]
#[serde(rename = "wrapper")]
struct OptionalWrapper<T: Display> {
    #[serde(
        serialize_with = "serialize_optional_csv",
        skip_serializing_if = "Option::is_none"
    )]
    value: Option<Vec<T>>,
}

#[test]
fn serializes_values_as_csv() {
    let wrapper = Wrapper {
        value: vec!["alpha", "beta", "gamma"],
    };

    let xml = quick_xml::se::to_string(&wrapper).expect("failed to serialize csv wrapper");

    assert_eq!(xml, "<wrapper><value>alpha,beta,gamma</value></wrapper>");
}

#[test]
fn serializes_display_values_as_csv() {
    let wrapper = Wrapper {
        value: vec![1, 2, 3],
    };

    let xml = quick_xml::se::to_string(&wrapper).expect("failed to serialize csv wrapper");

    assert_eq!(xml, "<wrapper><value>1,2,3</value></wrapper>");
}

#[test]
fn serializes_some_optional_values_as_csv() {
    let wrapper = OptionalWrapper {
        value: Some(vec!["alpha", "beta"]),
    };

    let xml = quick_xml::se::to_string(&wrapper).expect("failed to serialize optional csv wrapper");

    assert_eq!(xml, "<wrapper><value>alpha,beta</value></wrapper>");
}

#[test]
fn skips_none_optional_values() {
    let wrapper: OptionalWrapper<&str> = OptionalWrapper { value: None };

    let xml = quick_xml::se::to_string(&wrapper).expect("failed to serialize optional csv wrapper");

    assert_eq!(xml, "<wrapper/>");
}
