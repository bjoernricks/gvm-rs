// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use super::{AuthenticateRequest, AuthenticateResponse};

#[test]
fn serialize_authenticate_request() {
    let request = AuthenticateRequest::new("alice", "secret");

    let xml = quick_xml::se::to_string(&request).expect("failed to serialize authenticate request");

    assert_eq!(
        xml,
        "<authenticate><credentials><username>alice</username><password>secret</password></credentials></authenticate>"
    );
}

#[test]
fn serialize_authenticate_request_with_token() {
    let request = AuthenticateRequest::new("alice", "secret").with_token();

    let xml = quick_xml::se::to_string(&request)
        .expect("failed to serialize authenticate request with token");

    assert_eq!(
        xml,
        "<authenticate token=\"1\"><credentials><username>alice</username><password>secret</password></credentials></authenticate>"
    );
}

#[test]
fn deserialize_authenticate_response() {
    let xml = r#"<authenticate_response status="200" status_text="OK"><role>Admin</role><timezone>UTC</timezone><token>abc123</token></authenticate_response>"#;

    let response: AuthenticateResponse =
        quick_xml::de::from_str(xml).expect("failed to deserialize authenticate response");

    assert_eq!(response.status, 200);
    assert_eq!(response.status_text, "OK");
    assert_eq!(response.role, "Admin");
    assert_eq!(response.timezone, "UTC");
    assert_eq!(response.token.as_deref(), Some("abc123"));
}

#[test]
fn deserialize_authenticate_response_without_token() {
    let xml = r#"<authenticate_response status="200" status_text="OK"><role>Admin</role><timezone>UTC</timezone></authenticate_response>"#;

    let response: AuthenticateResponse =
        quick_xml::de::from_str(xml).expect("failed to deserialize authenticate response");

    assert_eq!(response.status, 200);
    assert_eq!(response.status_text, "OK");
    assert_eq!(response.role, "Admin");
    assert_eq!(response.timezone, "UTC");
    assert_eq!(response.token, None);
}
