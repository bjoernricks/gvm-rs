// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;

use super::{
    unwrap_csv_string, unwrap_optional_csv_string, unwrap_optional_uuid, unwrap_permissions,
    unwrap_uuid,
};
use crate::commands::entity::Permission;

#[derive(Debug, Deserialize)]
struct MacroEntry {
    name: String,
}

super::define_unwrap_vec_field!(unwrap_macro_entries, MacroEntries, entry, MacroEntry);
super::define_unwrap_optional_vec_field!(
    unwrap_optional_macro_entries,
    OptionalMacroEntries,
    entry,
    MacroEntry
);

#[derive(Debug, Deserialize)]
#[serde(rename = "wrapper")]
struct MacroVecWrapper {
    #[serde(default, deserialize_with = "unwrap_macro_entries")]
    entries: Vec<MacroEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "wrapper")]
struct MacroOptionalVecWrapper {
    #[serde(default, deserialize_with = "unwrap_optional_macro_entries")]
    entries: Option<Vec<MacroEntry>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "wrapper")]
struct CsvStringWrapper {
    #[serde(deserialize_with = "unwrap_csv_string")]
    value: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "wrapper")]
struct OptionalCsvStringWrapper {
    #[serde(default, deserialize_with = "unwrap_optional_csv_string")]
    value: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "wrapper")]
struct PermissionsWrapper {
    #[serde(deserialize_with = "unwrap_permissions")]
    permissions: Vec<Permission>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "wrapper")]
struct UuidWrapper {
    #[serde(deserialize_with = "unwrap_uuid")]
    id: uuid::Uuid,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "wrapper")]
struct OptionalUuidWrapper {
    #[serde(default, deserialize_with = "unwrap_optional_uuid")]
    id: Option<uuid::Uuid>,
}

#[test]
fn unwrap_csv_string_splits_and_trims_values() {
    let xml = r#"<wrapper><value>alpha, beta ,gamma</value></wrapper>"#;

    let wrapper: CsvStringWrapper =
        quick_xml::de::from_str(xml).expect("failed to deserialize csv wrapper");

    assert_eq!(wrapper.value, vec!["alpha", "beta", "gamma"]);
}

#[test]
fn unwrap_optional_csv_string_reads_some_and_none() {
    let with_value_xml = r#"<wrapper><value>one, two</value></wrapper>"#;
    let without_value_xml = r#"<wrapper></wrapper>"#;

    let with_value: OptionalCsvStringWrapper = quick_xml::de::from_str(with_value_xml)
        .expect("failed to deserialize optional csv wrapper");
    let without_value: OptionalCsvStringWrapper = quick_xml::de::from_str(without_value_xml)
        .expect("failed to deserialize optional csv wrapper");

    assert_eq!(
        with_value.value,
        Some(vec!["one".to_string(), "two".to_string()])
    );
    assert_eq!(without_value.value, None);
}

#[test]
fn unwrap_permissions_collects_permission_entries() {
    let xml = r#"<wrapper><permissions><permission><name>read</name></permission><permission><name>write</name></permission></permissions></wrapper>"#;

    let wrapper: PermissionsWrapper =
        quick_xml::de::from_str(xml).expect("failed to deserialize permissions wrapper");

    assert_eq!(wrapper.permissions.len(), 2);
    assert_eq!(wrapper.permissions[0].name, "read");
    assert_eq!(wrapper.permissions[1].name, "write");
}

#[test]
fn unwrap_uuid_maps_empty_to_nil_uuid() {
    let empty_xml = r#"<wrapper><id></id></wrapper>"#;

    let empty: UuidWrapper =
        quick_xml::de::from_str(empty_xml).expect("failed to deserialize uuid wrapper");

    assert_eq!(empty.id, uuid::Uuid::nil());
}

#[test]
fn unwrap_uuid_maps_zero_to_nil_uuid() {
    let zero_xml = r#"<wrapper><id>0</id></wrapper>"#;

    let zero: UuidWrapper =
        quick_xml::de::from_str(zero_xml).expect("failed to deserialize uuid wrapper");

    assert_eq!(zero.id, uuid::Uuid::nil());
}

#[test]
fn unwrap_optional_uuid_handles_missing_empty_zero_and_valid_values() {
    let missing_xml = r#"<wrapper></wrapper>"#;
    let empty_xml = r#"<wrapper><id></id></wrapper>"#;
    let zero_xml = r#"<wrapper><id>0</id></wrapper>"#;
    let valid_xml = r#"<wrapper><id>3db527c4-c3eb-41d8-b0e8-3f9752ac67f4</id></wrapper>"#;

    let missing: OptionalUuidWrapper =
        quick_xml::de::from_str(missing_xml).expect("failed to deserialize optional uuid wrapper");
    let empty: OptionalUuidWrapper =
        quick_xml::de::from_str(empty_xml).expect("failed to deserialize optional uuid wrapper");
    let zero: OptionalUuidWrapper =
        quick_xml::de::from_str(zero_xml).expect("failed to deserialize optional uuid wrapper");
    let valid: OptionalUuidWrapper =
        quick_xml::de::from_str(valid_xml).expect("failed to deserialize optional uuid wrapper");

    assert_eq!(missing.id, None);
    assert_eq!(empty.id, None);
    assert_eq!(zero.id, Some(uuid::Uuid::nil()));
    assert_eq!(
        valid.id,
        Some(
            "3db527c4-c3eb-41d8-b0e8-3f9752ac67f4"
                .parse::<uuid::Uuid>()
                .expect("invalid test uuid")
        )
    );
}

#[test]
fn macro_unwrap_vec_field_reads_entries_and_defaults_to_empty() {
    let with_entries_xml = r#"<wrapper><entries><entry><name>one</name></entry><entry><name>two</name></entry></entries></wrapper>"#;
    let without_entries_xml = r#"<wrapper></wrapper>"#;

    let with_entries: MacroVecWrapper =
        quick_xml::de::from_str(with_entries_xml).expect("failed to deserialize macro vec wrapper");
    let without_entries: MacroVecWrapper = quick_xml::de::from_str(without_entries_xml)
        .expect("failed to deserialize macro vec wrapper");

    assert_eq!(with_entries.entries.len(), 2);
    assert_eq!(with_entries.entries[0].name, "one");
    assert_eq!(with_entries.entries[1].name, "two");
    assert!(without_entries.entries.is_empty());
}

#[test]
fn macro_unwrap_optional_vec_field_reads_entries_and_none_when_missing() {
    let with_entries_xml =
        r#"<wrapper><entries><entry><name>one</name></entry></entries></wrapper>"#;
    let without_entries_xml = r#"<wrapper></wrapper>"#;

    let with_entries: MacroOptionalVecWrapper = quick_xml::de::from_str(with_entries_xml)
        .expect("failed to deserialize macro optional vec wrapper");
    let without_entries: MacroOptionalVecWrapper = quick_xml::de::from_str(without_entries_xml)
        .expect("failed to deserialize macro optional vec wrapper");

    let entries = with_entries
        .entries
        .expect("expected optional entries to be present");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "one");
    assert!(without_entries.entries.is_none());
}
