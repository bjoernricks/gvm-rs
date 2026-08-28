// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use super::{GetTargetsRequest, GetTargetsResponse};
use crate::commands::entity::{CollectionCounts, KeywordRelation};

fn sample_target_xml(alive_test: &str) -> String {
    format!(
        "<target id=\"74f3ff0c-8be7-4c85-bf6e-001dc0719d00\"><name>Localhost</name><comment></comment><creation_time>2026-04-08T07:15:09Z</creation_time><modification_time>2026-04-08T07:15:09Z</modification_time><owner><name>admin</name></owner><writable>1</writable><in_use>1</in_use><permissions><permission><name>Everything</name></permission></permissions><hosts>127.0.0.1</hosts><exclude_hosts></exclude_hosts><max_hosts>1</max_hosts><ssh_credential id=\"\"><name></name><trash>0</trash></ssh_credential><smb_credential id=\"\"><name></name><trash>0</trash></smb_credential><esxi_credential id=\"\"><name></name><trash>0</trash></esxi_credential><krb5_credential id=\"\"><name></name><trash>0</trash></krb5_credential><snmp_credential id=\"\"><name></name><trash>0</trash></snmp_credential><ssh_elevate_credential id=\"\"><name></name><trash>0</trash></ssh_elevate_credential><port_list id=\"33d0cd82-57c6-11e1-8ed1-406186ea4fc5\"><name>All IANA assigned TCP</name><trash>0</trash></port_list><alive_tests><alive_test>{alive_test}</alive_test></alive_tests><reverse_lookup_only>0</reverse_lookup_only><reverse_lookup_unify>0</reverse_lookup_unify><allow_simultaneous_ips>1</allow_simultaneous_ips></target>"
    )
}

#[test]
fn serialize_get_targets_request() {
    let request = GetTargetsRequest::new();

    let xml = quick_xml::se::to_string(&request).expect("failed to serialize get_targets request");

    assert_eq!(xml, "<get_targets/>");
}

#[test]
fn serialize_get_targets_request_with_options() {
    let request = GetTargetsRequest::new()
        .with_details()
        .with_filter_id("abc")
        .with_trash()
        .with_tasks();

    let xml = quick_xml::se::to_string(&request)
        .expect("failed to serialize get_targets request with options");

    assert_eq!(
        xml,
        "<get_targets details=\"1\" filt_id=\"abc\" trash=\"1\" tasks=\"1\"/>"
    );
}

#[test]
fn deserialize_get_targets_response_filter() {
    let xml = format!(
        "<get_targets_response status=\"200\" status_text=\"OK\">{}<filters id=\"\"><term>first=1 rows=10 sort=name</term><keywords><keyword><column>first</column><relation>=</relation><value>1</value></keyword><keyword><column>rows</column><relation>=</relation><value>10</value></keyword><keyword><column>sort</column><relation>=</relation><value>name</value></keyword></keywords></filters></get_targets_response>",
        sample_target_xml("Scan Config Default")
    );

    let response: GetTargetsResponse =
        quick_xml::de::from_str(&xml).expect("failed to deserialize get_targets_response");

    assert_eq!(response.status, 200);
    assert_eq!(response.status_text, "OK");
    assert_eq!(response.filter.id, None);
    assert_eq!(response.filter.name, None);
    assert_eq!(response.filter.term, "first=1 rows=10 sort=name");
    assert_eq!(response.filter.keywords.len(), 3);
    assert_eq!(response.filter.keywords[0].column, "first");
    assert_eq!(response.filter.keywords[0].relation, KeywordRelation::Eq);
    assert_eq!(response.filter.keywords[0].value, "1");

    assert_eq!(response.targets.len(), 1);
    assert_eq!(response.targets[0].name, "Localhost");
    assert_eq!(response.targets[0].alive_tests.len(), 1);
    assert!(matches!(
        response.targets[0].alive_tests[0],
        super::super::AliveTest::ScanConfigDefault
    ));
    assert!(response.targets[0].port_range.is_none());
    assert!(response.targets[0].tasks.is_empty());
}

#[test]
fn deserialize_get_targets_response_without_targets() {
    let xml = "<get_targets_response status=\"200\" status_text=\"OK\"><filters id=\"\"><term>first=1 rows=10 sort=name</term></filters></get_targets_response>";

    let response: GetTargetsResponse =
        quick_xml::de::from_str(xml).expect("failed to deserialize get_targets_response");

    assert!(response.targets.is_empty());
}

#[test]
fn deserialize_get_targets_response_collection_counts() {
    let xml = format!(
        "<get_targets_response status=\"200\" status_text=\"OK\">{}<filters id=\"\"><term></term></filters><targets start=\"7\" max=\"42\"/><target_count>9<filtered>5</filtered><page>2</page></target_count></get_targets_response>",
        sample_target_xml("Scan Config Default")
    );

    let response: GetTargetsResponse =
        quick_xml::de::from_str(&xml).expect("failed to deserialize get_targets_response");

    assert_eq!(
        *response.counts,
        CollectionCounts {
            first: 7,
            rows: 42,
            all: 9,
            filtered: 5,
            length: 2
        }
    );
}
