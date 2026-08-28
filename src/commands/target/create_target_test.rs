// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use super::CreateTargetRequest;
use crate::commands::target::AliveTest;
use uuid::Uuid;

#[test]
fn serialize_create_target_request_hosts_as_csv() {
    let request = CreateTargetRequest::from_port_list("target", Uuid::nil())
        .with_hosts(vec!["host-a", "host-b"]);

    let xml =
        quick_xml::se::to_string(&request).expect("failed to serialize create_target request");

    assert!(xml.contains("<hosts>host-a,host-b</hosts>"));
}

#[test]
fn serialize_create_target_request_exclude_hosts_as_csv() {
    let request = CreateTargetRequest::from_port_list("target", Uuid::nil())
        .with_exclude_hosts(vec!["excluded-a", "excluded-b"]);

    let xml =
        quick_xml::se::to_string(&request).expect("failed to serialize create_target request");

    assert!(xml.contains("<exclude_hosts>excluded-a,excluded-b</exclude_hosts>"));
}

#[test]
fn serialize_create_target_request_without_exclude_hosts() {
    let request = CreateTargetRequest::from_port_list("target", Uuid::nil());

    let xml =
        quick_xml::se::to_string(&request).expect("failed to serialize create_target request");

    assert!(!xml.contains("<exclude_hosts"));
}

#[test]
fn serialize_create_target_request_ssh_credential_as_id_attribute() {
    let credential_id =
        Uuid::parse_str("3db527c4-c3eb-41d8-b0e8-3f9e2f1d0f90").expect("valid credential UUID");
    let request = CreateTargetRequest::from_port_list("target", Uuid::nil())
        .with_ssh_credential(credential_id)
        .with_smb_credential(credential_id)
        .with_esxi_credential(credential_id)
        .with_krb5_credential(credential_id)
        .with_snmp_credential(credential_id)
        .with_ssh_elevate_credential(credential_id);

    let xml =
        quick_xml::se::to_string(&request).expect("failed to serialize create_target request");

    assert!(xml.contains(&format!("<ssh_credential id=\"{credential_id}\"/>")));
    assert!(xml.contains(&format!("<smb_credential id=\"{credential_id}\"/>")));
    assert!(xml.contains(&format!("<esxi_credential id=\"{credential_id}\"/>")));
    assert!(xml.contains(&format!("<krb5_credential id=\"{credential_id}\"/>")));
    assert!(xml.contains(&format!("<snmp_credential id=\"{credential_id}\"/>")));
    assert!(xml.contains(&format!("<ssh_elevate_credential id=\"{credential_id}\"/>")));
}

#[test]
fn serialize_create_target_request_port_list_as_id_attribute() {
    let port_list_id =
        Uuid::parse_str("33d0cd82-57c6-11e1-8ed1-406186ea4fc5").expect("valid port list UUID");
    let request = CreateTargetRequest::from_port_list("target", port_list_id);

    let xml =
        quick_xml::se::to_string(&request).expect("failed to serialize create_target request");

    assert!(xml.contains(&format!("<port_list id=\"{port_list_id}\"/>")));
}

#[test]
fn serialize_create_target_request_port_ranges_as_csv() {
    let request = CreateTargetRequest::from_port_ranges("target", vec!["1-1024", "8080"]);

    let xml =
        quick_xml::se::to_string(&request).expect("failed to serialize create_target request");

    assert!(xml.contains("<port_range>1-1024,8080</port_range>"));
}

#[test]
fn serialize_create_target_request_boolean_values_as_integers() {
    let default_request = CreateTargetRequest::from_port_list("target", Uuid::nil());
    let default_xml = quick_xml::se::to_string(&default_request)
        .expect("failed to serialize create_target request");

    assert!(default_xml.contains("<reverse_lookup_only>0</reverse_lookup_only>"));
    assert!(default_xml.contains("<reverse_lookup_unify>0</reverse_lookup_unify>"));
    assert!(default_xml.contains("<allow_simultaneous_ips>0</allow_simultaneous_ips>"));

    let enabled_request = CreateTargetRequest::from_port_list("target", Uuid::nil())
        .with_reverse_lookup_only(true)
        .with_reverse_lookup_unify(true)
        .with_allow_simultaneous_ips(true);
    let enabled_xml = quick_xml::se::to_string(&enabled_request)
        .expect("failed to serialize create_target request");

    assert!(enabled_xml.contains("<reverse_lookup_only>1</reverse_lookup_only>"));
    assert!(enabled_xml.contains("<reverse_lookup_unify>1</reverse_lookup_unify>"));
    assert!(enabled_xml.contains("<allow_simultaneous_ips>1</allow_simultaneous_ips>"));
}

#[test]
fn serialize_create_target_request_alive_tests_as_nested_elements() {
    let request = CreateTargetRequest::from_port_list("target", Uuid::nil())
        .with_alive_tests(vec![AliveTest::ScanConfigDefault, AliveTest::IcmpPing]);

    let xml =
        quick_xml::se::to_string(&request).expect("failed to serialize create_target request");

    assert!(xml.contains(
        "<alive_tests><alive_test>Scan Config Default</alive_test><alive_test>ICMP Ping</alive_test></alive_tests>"
    ));
}

#[test]
fn create_target_request_with_builders_sets_fields() {
    let credential_id = Uuid::nil();
    let request = CreateTargetRequest::from_port_list("target", Uuid::nil())
        .with_comment("comment")
        .with_hosts(vec!["host-a", "host-b"])
        .with_exclude_hosts(vec!["excluded-host"])
        .with_ssh_credential(credential_id)
        .with_smb_credential(credential_id)
        .with_esxi_credential(credential_id)
        .with_krb5_credential(credential_id)
        .with_snmp_credential(credential_id)
        .with_ssh_elevate_credential(credential_id)
        .with_alive_tests(vec![AliveTest::ArpPing, AliveTest::IcmpPing])
        .with_reverse_lookup_only(true)
        .with_reverse_lookup_unify(true)
        .with_allow_simultaneous_ips(true);

    assert_eq!(request.comment.as_deref(), Some("comment"));
    assert_eq!(request.hosts, vec!["host-a", "host-b"]);
    assert_eq!(
        request.exclude_hosts,
        Some(vec!["excluded-host".to_string()])
    );
    assert_eq!(
        request
            .ssh_credential
            .as_ref()
            .map(|reference| reference.id),
        Some(credential_id)
    );
    assert_eq!(
        request
            .smb_credential
            .as_ref()
            .map(|reference| reference.id),
        Some(credential_id)
    );
    assert_eq!(
        request
            .esxi_credential
            .as_ref()
            .map(|reference| reference.id),
        Some(credential_id)
    );
    assert_eq!(
        request
            .krb5_credential
            .as_ref()
            .map(|reference| reference.id),
        Some(credential_id)
    );
    assert_eq!(
        request
            .snmp_credential
            .as_ref()
            .map(|reference| reference.id),
        Some(credential_id)
    );
    assert_eq!(
        request
            .ssh_elevate_credential
            .as_ref()
            .map(|reference| reference.id),
        Some(credential_id)
    );
    assert!(matches!(request.alive_tests[0], AliveTest::ArpPing));
    assert!(matches!(request.alive_tests[1], AliveTest::IcmpPing));
    assert!(request.reverse_lookup_only);
    assert!(request.reverse_lookup_unify);
    assert!(request.allow_simultaneous_ips);
}
