// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod parse_u32_or_zero {
    #[test]
    fn parses_valid_number() {
        assert_eq!(super::super::parse_u32_or_zero("42"), 42);
    }

    #[test]
    fn trims_whitespace() {
        assert_eq!(super::super::parse_u32_or_zero(" 7 "), 7);
    }

    #[test]
    fn returns_zero_for_empty_string() {
        assert_eq!(super::super::parse_u32_or_zero(""), 0);
    }

    #[test]
    fn returns_zero_for_invalid_input() {
        assert_eq!(super::super::parse_u32_or_zero("not a number"), 0);
    }
}

mod unwrap_csv_string {
    use serde::Deserialize;

    use super::super::unwrap_csv_string;

    #[derive(Debug, Deserialize)]
    #[serde(rename = "wrapper")]
    struct Wrapper {
        #[serde(deserialize_with = "unwrap_csv_string")]
        value: Vec<String>,
    }

    #[test]
    fn splits_and_trims_values() {
        let xml = r#"<wrapper><value>alpha, beta ,gamma</value></wrapper>"#;

        let wrapper: Wrapper =
            quick_xml::de::from_str(xml).expect("failed to deserialize csv wrapper");

        assert_eq!(wrapper.value, vec!["alpha", "beta", "gamma"]);
    }
}

mod unwrap_optional_csv_string {
    use serde::Deserialize;

    use super::super::unwrap_optional_csv_string;

    #[derive(Debug, Deserialize)]
    #[serde(rename = "wrapper")]
    struct Wrapper {
        #[serde(default, deserialize_with = "unwrap_optional_csv_string")]
        value: Option<Vec<String>>,
    }

    #[test]
    fn reads_some_and_none() {
        let with_value_xml = r#"<wrapper><value>one, two</value></wrapper>"#;
        let without_value_xml = r#"<wrapper></wrapper>"#;

        let with_value: Wrapper = quick_xml::de::from_str(with_value_xml)
            .expect("failed to deserialize optional csv wrapper");
        let without_value: Wrapper = quick_xml::de::from_str(without_value_xml)
            .expect("failed to deserialize optional csv wrapper");

        assert_eq!(
            with_value.value,
            Some(vec!["one".to_string(), "two".to_string()])
        );
        assert_eq!(without_value.value, None);
    }
}

mod unwrap_permissions {
    use serde::Deserialize;

    use super::super::unwrap_permissions;
    use crate::commands::entity::Permission;

    #[derive(Debug, Deserialize)]
    #[serde(rename = "wrapper")]
    struct Wrapper {
        #[serde(deserialize_with = "unwrap_permissions")]
        permissions: Vec<Permission>,
    }

    #[test]
    fn collects_permission_entries() {
        let xml = r#"<wrapper><permissions><permission><name>read</name></permission><permission><name>write</name></permission></permissions></wrapper>"#;

        let wrapper: Wrapper =
            quick_xml::de::from_str(xml).expect("failed to deserialize permissions wrapper");

        assert_eq!(wrapper.permissions.len(), 2);
        assert_eq!(wrapper.permissions[0].name, "read");
        assert_eq!(wrapper.permissions[1].name, "write");
    }
}

mod unwrap_uuid {
    use serde::Deserialize;

    use super::super::unwrap_uuid;

    #[derive(Debug, Deserialize)]
    #[serde(rename = "wrapper")]
    struct Wrapper {
        #[serde(deserialize_with = "unwrap_uuid")]
        id: uuid::Uuid,
    }

    #[test]
    fn maps_empty_to_nil_uuid() {
        let xml = r#"<wrapper><id></id></wrapper>"#;

        let wrapper: Wrapper =
            quick_xml::de::from_str(xml).expect("failed to deserialize uuid wrapper");

        assert_eq!(wrapper.id, uuid::Uuid::nil());
    }

    #[test]
    fn maps_zero_to_nil_uuid() {
        let xml = r#"<wrapper><id>0</id></wrapper>"#;

        let wrapper: Wrapper =
            quick_xml::de::from_str(xml).expect("failed to deserialize uuid wrapper");

        assert_eq!(wrapper.id, uuid::Uuid::nil());
    }
}

mod unwrap_optional_uuid {
    use serde::Deserialize;

    use super::super::unwrap_optional_uuid;

    #[derive(Debug, Deserialize)]
    #[serde(rename = "wrapper")]
    struct Wrapper {
        #[serde(default, deserialize_with = "unwrap_optional_uuid")]
        id: Option<uuid::Uuid>,
    }

    #[test]
    fn handles_missing_empty_zero_and_valid_values() {
        let missing_xml = r#"<wrapper></wrapper>"#;
        let empty_xml = r#"<wrapper><id></id></wrapper>"#;
        let zero_xml = r#"<wrapper><id>0</id></wrapper>"#;
        let valid_xml = r#"<wrapper><id>3db527c4-c3eb-41d8-b0e8-3f9752ac67f4</id></wrapper>"#;

        let missing: Wrapper = quick_xml::de::from_str(missing_xml)
            .expect("failed to deserialize optional uuid wrapper");
        let empty: Wrapper = quick_xml::de::from_str(empty_xml)
            .expect("failed to deserialize optional uuid wrapper");
        let zero: Wrapper =
            quick_xml::de::from_str(zero_xml).expect("failed to deserialize optional uuid wrapper");
        let valid: Wrapper = quick_xml::de::from_str(valid_xml)
            .expect("failed to deserialize optional uuid wrapper");

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
}

mod define_unwrap_vec_field {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Entry {
        name: String,
    }

    super::super::define_unwrap_vec_field!(unwrap_entries, Entries, entry, Entry);

    #[derive(Debug, Deserialize)]
    struct OptionalEntries {
        #[serde(default)]
        entry: Option<Vec<Entry>>,
    }

    fn unwrap_optional_entries<'de, D>(deserializer: D) -> Result<Option<Vec<Entry>>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(OptionalEntries::deserialize(deserializer)?.entry)
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename = "wrapper")]
    struct VecWrapper {
        #[serde(default, deserialize_with = "unwrap_entries")]
        entries: Vec<Entry>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename = "wrapper")]
    struct OptionalVecWrapper {
        #[serde(default, deserialize_with = "unwrap_optional_entries")]
        entries: Option<Vec<Entry>>,
    }

    #[test]
    fn reads_entries_and_defaults_to_empty() {
        let with_entries_xml = r#"<wrapper><entries><entry><name>one</name></entry><entry><name>two</name></entry></entries></wrapper>"#;
        let without_entries_xml = r#"<wrapper></wrapper>"#;

        let with_entries: VecWrapper = quick_xml::de::from_str(with_entries_xml)
            .expect("failed to deserialize macro vec wrapper");
        let without_entries: VecWrapper = quick_xml::de::from_str(without_entries_xml)
            .expect("failed to deserialize macro vec wrapper");

        assert_eq!(with_entries.entries.len(), 2);
        assert_eq!(with_entries.entries[0].name, "one");
        assert_eq!(with_entries.entries[1].name, "two");
        assert!(without_entries.entries.is_empty());
    }

    #[test]
    fn optional_reads_entries_and_none_when_missing() {
        let with_entries_xml =
            r#"<wrapper><entries><entry><name>one</name></entry></entries></wrapper>"#;
        let without_entries_xml = r#"<wrapper></wrapper>"#;

        let with_entries: OptionalVecWrapper = quick_xml::de::from_str(with_entries_xml)
            .expect("failed to deserialize macro optional vec wrapper");
        let without_entries: OptionalVecWrapper = quick_xml::de::from_str(without_entries_xml)
            .expect("failed to deserialize macro optional vec wrapper");

        let entries = with_entries
            .entries
            .expect("expected optional entries to be present");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "one");
        assert!(without_entries.entries.is_none());
    }
}

mod define_collection_counts_deserializer {
    use serde::Deserialize;

    use crate::commands::entity::CollectionCounts;

    super::super::define_collection_counts_deserializer!(DummyCounts, "dummies", "dummy_count");

    #[derive(Debug, Deserialize)]
    #[serde(rename = "dummy_response")]
    struct Wrapper {
        #[serde(flatten)]
        counts: DummyCounts,
    }

    #[test]
    fn maps_list_and_count_elements() {
        let xml = r#"<dummy_response><dummies start="7" max="42"/><dummy_count>9<filtered>5</filtered><page>2</page></dummy_count></dummy_response>"#;

        let wrapper: Wrapper =
            quick_xml::de::from_str(xml).expect("failed to deserialize counts wrapper");

        assert_eq!(
            *wrapper.counts,
            CollectionCounts {
                first: 7,
                rows: 42,
                all: 9,
                filtered: 5,
                length: 2
            }
        );
    }
}
