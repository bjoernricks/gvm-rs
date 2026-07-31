// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use super::{GetVersionRequest, GetVersionResponse};

#[test]
fn serialize_get_version_request() {
    let request = GetVersionRequest;

    let xml = quick_xml::se::to_string(&request).expect("failed to serialize get_version request");

    assert_eq!(xml, "<get_version/>");
}

#[test]
fn deserialize_get_version_response() {
    let xml = r#"<get_version_response status="200" status_text="OK"><version>23.12.1</version></get_version_response>"#;

    let response: GetVersionResponse =
        quick_xml::de::from_str(xml).expect("failed to deserialize get_version response");

    assert_eq!(response.status, 200);
    assert_eq!(response.status_text, "OK");
    assert_eq!(response.version, "23.12.1");
}
